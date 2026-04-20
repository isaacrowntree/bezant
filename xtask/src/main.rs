//! Dev workflow tasks that live outside the published crates.
//!
//! Runs progenitor directly against arbitrary spec slices without going through
//! the `bezant-api` build script, so iteration on spec normalization is orders
//! of magnitude faster.
//!
//! Subcommands:
//!   - `codegen` — generate once against the vendored spec (like build.rs would)
//!   - `probe`   — parse + run progenitor against a spec file, report pass/fail
//!   - `bisect-schemas` — drop one schema at a time, report which ones unblock codegen

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use serde_json::Value;

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Run progenitor against the vendored spec once.
    Codegen {
        /// Path to the spec JSON to generate from.
        #[arg(long, default_value = "crates/bezant-spec/ibkr-openapi.json")]
        spec: PathBuf,
        /// Output file for the generated Rust code.
        #[arg(long, default_value = "target/xtask-generated.rs")]
        out: PathBuf,
    },

    /// Attempt codegen and print the short pass/fail result.
    Probe {
        #[arg(long, default_value = "crates/bezant-spec/ibkr-openapi.json")]
        spec: PathBuf,
    },

    /// Iterate through schemas, dropping each in turn, and report which ones
    /// are load-bearing for the codegen failure.
    BisectSchemas {
        #[arg(long, default_value = "crates/bezant-spec/ibkr-openapi.json")]
        spec: PathBuf,
        /// Only test this substring against schema names (case-insensitive).
        #[arg(long)]
        filter: Option<String>,
    },

    /// Run codegen against each schema in isolation (empty paths, only that
    /// schema plus its transitively-referenced friends). Reports every schema
    /// whose isolated codegen fails.
    IsolateSchemas {
        #[arg(long, default_value = "crates/bezant-spec/ibkr-openapi.json")]
        spec: PathBuf,
        #[arg(long)]
        filter: Option<String>,
    },

    /// For each operation, try codegen with *only that operation* and report
    /// which ones individually fail. Useful for finding the specific operation
    /// tripping an assertion.
    IsolateOps {
        #[arg(long, default_value = "crates/bezant-spec/ibkr-openapi.json")]
        spec: PathBuf,
        #[arg(long)]
        filter: Option<String>,
    },
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    let result = match cli.cmd {
        Cmd::Codegen { spec, out } => codegen(&spec, &out),
        Cmd::Probe { spec } => probe(&spec),
        Cmd::BisectSchemas { spec, filter } => bisect_schemas(&spec, filter.as_deref()),
        Cmd::IsolateSchemas { spec, filter } => isolate_schemas(&spec, filter.as_deref()),
        Cmd::IsolateOps { spec, filter } => isolate_ops(&spec, filter.as_deref()),
    };
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("xtask: {e:#}");
            ExitCode::FAILURE
        }
    }
}

fn load_spec(path: &Path) -> Result<Value> {
    let bytes = std::fs::read(path).with_context(|| format!("read {}", path.display()))?;
    serde_json::from_slice(&bytes).context("parse spec as JSON")
}

fn try_codegen(spec_json: &Value) -> Result<String> {
    // typify (progenitor's schema engine) panics on missing $refs / unknown
    // constructs. We catch the panic so bisection can continue rather than
    // crashing the process on the first broken subset.
    let spec_clone = spec_json.clone();
    let result =
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(move || -> Result<String> {
            let oapi: openapiv3::OpenAPI =
                serde_json::from_value(spec_clone).context("parse as openapiv3")?;
            let mut generator = progenitor::Generator::default();
            let tokens = generator
                .generate_tokens(&oapi)
                .map_err(|e| anyhow::anyhow!("progenitor: {e}"))?;
            let ast = syn::parse2::<syn::File>(tokens).context("syn::parse2")?;
            Ok(prettyplease::unparse(&ast))
        }));
    match result {
        Ok(inner) => inner,
        Err(panic) => {
            let msg = if let Some(s) = panic.downcast_ref::<&str>() {
                (*s).to_string()
            } else if let Some(s) = panic.downcast_ref::<String>() {
                s.clone()
            } else {
                "<non-string panic>".into()
            };
            anyhow::bail!("panic in codegen: {msg}")
        }
    }
}

fn codegen(spec: &Path, out: &Path) -> Result<()> {
    let spec_json = load_spec(spec)?;
    let pretty = try_codegen(&spec_json)?;
    if let Some(parent) = out.parent() {
        std::fs::create_dir_all(parent).ok();
    }
    std::fs::write(out, pretty).with_context(|| format!("write {}", out.display()))?;
    println!("wrote {}", out.display());
    Ok(())
}

fn probe(spec: &Path) -> Result<()> {
    let spec_json = load_spec(spec)?;
    match try_codegen(&spec_json) {
        Ok(src) => {
            println!("codegen OK ({} bytes)", src.len());
            Ok(())
        }
        Err(e) => {
            println!("codegen FAIL: {e}");
            anyhow::bail!("codegen failed")
        }
    }
}

fn bisect_schemas(spec: &Path, filter: Option<&str>) -> Result<()> {
    let full = load_spec(spec)?;

    // Baseline: does full spec fail?
    let full_err = try_codegen(&full).err();
    if full_err.is_none() {
        println!("full spec codegen is OK — nothing to bisect.");
        return Ok(());
    }
    println!("baseline: FAIL ({})", full_err.unwrap());

    // Enumerate schemas and test each drop
    let schemas = full
        .get("components")
        .and_then(|c| c.get("schemas"))
        .and_then(Value::as_object)
        .cloned()
        .unwrap_or_default();
    if schemas.is_empty() {
        anyhow::bail!("no schemas in spec");
    }

    let mut offenders: BTreeMap<String, String> = BTreeMap::new();
    let names: Vec<_> = schemas
        .keys()
        .cloned()
        .filter(|n| filter.map_or(true, |f| n.to_lowercase().contains(&f.to_lowercase())))
        .collect();
    println!(
        "testing {} schemas (dropping one at a time)...",
        names.len()
    );

    // We replace the removed schema with an empty `{}` definition rather than
    // deleting its key — preserves $ref resolvability for other schemas/paths.
    for (i, name) in names.iter().enumerate() {
        let mut subset = full.clone();
        let schemas_mut = subset
            .pointer_mut("/components/schemas")
            .and_then(Value::as_object_mut)
            .expect("schemas exist");
        schemas_mut.insert(name.clone(), serde_json::json!({}));
        let outcome = try_codegen(&subset);
        match &outcome {
            Ok(_) => {
                offenders.insert(name.clone(), "stubbing to {} unblocks codegen".into());
                println!("  [{:3}/{}] UNBLOCKS: {name}", i + 1, names.len());
            }
            Err(e) => {
                let msg = e.to_string();
                // Report the first non-baseline error as potentially interesting.
                if !msg.contains("value does not conform") && !msg.contains("panic in codegen") {
                    println!(
                        "  [{:3}/{}] {name}: different error: {}",
                        i + 1,
                        names.len(),
                        snippet(&msg)
                    );
                }
                if (i + 1) % 50 == 0 {
                    println!("  [{:3}/{}] progress...", i + 1, names.len());
                }
            }
        }
    }

    println!();
    println!("===== summary =====");
    if offenders.is_empty() {
        println!("no single-schema unblocks — the error is in combinations or paths/components.");
    } else {
        for (name, note) in offenders {
            println!("  {name}: {note}");
        }
    }
    Ok(())
}

fn isolate_schemas(spec: &Path, filter: Option<&str>) -> Result<()> {
    let full = load_spec(spec)?;
    let schemas = full
        .get("components")
        .and_then(|c| c.get("schemas"))
        .and_then(Value::as_object)
        .cloned()
        .unwrap_or_default();

    let names: Vec<_> = schemas
        .keys()
        .cloned()
        .filter(|n| filter.map_or(true, |f| n.to_lowercase().contains(&f.to_lowercase())))
        .collect();

    println!("isolation-testing {} schemas...", names.len());
    let mut failures: Vec<(String, String)> = Vec::new();

    // Collect *every* schema so $refs from the isolated schema can still
    // resolve into placeholder types. We replace all non-target schemas with
    // empty objects so they're cheap to process.
    for (i, name) in names.iter().enumerate() {
        let mut subset = full.clone();
        subset
            .as_object_mut()
            .unwrap()
            .insert("paths".into(), Value::Object(Default::default()));

        let target_schema = schemas.get(name).unwrap().clone();
        let schemas_mut = subset
            .pointer_mut("/components/schemas")
            .and_then(Value::as_object_mut)
            .expect("schemas exist");
        // Stub every other schema, keep the target verbatim
        for (n, _) in schemas.iter() {
            if n != name {
                schemas_mut.insert(n.clone(), serde_json::json!({}));
            }
        }
        schemas_mut.insert(name.clone(), target_schema);

        match try_codegen(&subset) {
            Ok(_) => {}
            Err(e) => {
                let msg = e.to_string();
                println!(
                    "  [{:3}/{}] FAIL {name}: {}",
                    i + 1,
                    names.len(),
                    snippet(&msg)
                );
                failures.push((name.clone(), msg));
            }
        }
    }

    println!();
    println!("===== {} schemas fail in isolation =====", failures.len());
    for (name, msg) in &failures {
        println!("  {name}: {}", snippet(msg));
    }
    Ok(())
}

fn isolate_ops(spec: &Path, filter: Option<&str>) -> Result<()> {
    let full = load_spec(spec)?;
    let paths = full
        .get("paths")
        .and_then(Value::as_object)
        .cloned()
        .unwrap_or_default();

    // Enumerate (path, method) pairs
    let mut ops: Vec<(String, String)> = Vec::new();
    for (path, item) in paths.iter() {
        if let Some(item_map) = item.as_object() {
            for (method, _) in item_map {
                let m = method.to_lowercase();
                if matches!(
                    m.as_str(),
                    "get" | "post" | "put" | "delete" | "patch" | "options" | "head" | "trace"
                ) {
                    if filter.map_or(true, |f| {
                        format!("{} {}", method, path)
                            .to_lowercase()
                            .contains(&f.to_lowercase())
                    }) {
                        ops.push((path.clone(), method.clone()));
                    }
                }
            }
        }
    }

    println!("isolation-testing {} operations...", ops.len());
    let mut failures = Vec::new();

    for (i, (path, method)) in ops.iter().enumerate() {
        let mut subset = full.clone();
        // Clear all paths except this one-op
        let paths_mut = subset
            .pointer_mut("/paths")
            .and_then(Value::as_object_mut)
            .unwrap();
        paths_mut.clear();

        // Rebuild the single-op path entry
        let orig_item = paths.get(path).unwrap().clone();
        let orig_map = orig_item.as_object().unwrap();
        let mut kept = serde_json::Map::new();
        if let Some(op) = orig_map.get(method) {
            kept.insert(method.clone(), op.clone());
        }
        // Keep path-level "parameters" if present
        if let Some(p) = orig_map.get("parameters") {
            kept.insert("parameters".into(), p.clone());
        }
        paths_mut.insert(path.clone(), Value::Object(kept));

        match try_codegen(&subset) {
            Ok(_) => {}
            Err(e) => {
                let msg = e.to_string();
                let short = snippet(&msg);
                println!(
                    "  [{:3}/{}] FAIL {} {}: {}",
                    i + 1,
                    ops.len(),
                    method.to_uppercase(),
                    path,
                    short
                );
                failures.push((path.clone(), method.clone(), msg));
            }
        }
    }

    println!();
    println!(
        "===== {} operations fail in isolation =====",
        failures.len()
    );
    Ok(())
}

fn snippet(s: &str) -> String {
    let trimmed = s.replace('\n', " ");
    if trimmed.len() > 120 {
        format!("{}…", &trimmed[..120])
    } else {
        trimmed
    }
}
