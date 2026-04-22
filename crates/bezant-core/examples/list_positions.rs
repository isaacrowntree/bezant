//! List every position in an account, paginated across every page the
//! Gateway reports.
//!
//! ```text
//! IBKR_ACCOUNT_ID=DU123456 \
//!     cargo run -p bezant-core --example list_positions
//! ```

use std::time::Duration;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let gateway_url = std::env::var("IBKR_GATEWAY_URL")
        .unwrap_or_else(|_| "https://localhost:5000/v1/api".to_string());
    let account_id = std::env::var("IBKR_ACCOUNT_ID")
        .map_err(|_| anyhow::anyhow!("set IBKR_ACCOUNT_ID (e.g. DU123456 for paper)"))?;

    let client = bezant::Client::builder(&gateway_url)
        .accept_invalid_certs(true)
        .timeout(Duration::from_secs(30))
        .build()?;

    // Keep the session alive while we work — stops automatically when the
    // returned handle is dropped at the end of main.
    let _keepalive = client.spawn_keepalive(Duration::from_secs(60));

    let positions = client.all_positions(&account_id).await?;
    println!("{} positions in {account_id}", positions.len());

    for p in positions {
        let qty = fmt_opt(p.position);
        let cost = fmt_opt(p.avg_cost);
        let value = fmt_opt(p.mkt_value);
        println!(
            "  {:<10} qty={qty:>10} avgCost={cost:>10} mktVal={value:>10}",
            p.contract_desc.as_deref().unwrap_or("?"),
        );
    }
    Ok(())
}

fn fmt_opt(value: Option<f64>) -> String {
    value.map(|v| format!("{v:.2}")).unwrap_or_else(|| "-".into())
}
