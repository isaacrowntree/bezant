//! Subscribe to real-time L1 quotes via the CPAPI WebSocket and print
//! every tick until the user hits Ctrl-C.
//!
//! ```text
//! IBKR_SYMBOL=AAPL cargo run -p bezant-core --example stream_quotes
//! ```

use std::time::Duration;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let gateway_url = std::env::var("IBKR_GATEWAY_URL")
        .unwrap_or_else(|_| "https://localhost:5000/v1/api".to_string());
    let symbol = std::env::var("IBKR_SYMBOL").unwrap_or_else(|_| "AAPL".to_string());

    let client = bezant::Client::builder(&gateway_url)
        .accept_invalid_certs(true)
        .timeout(Duration::from_secs(30))
        .build()?;

    let _keepalive = client.spawn_keepalive(Duration::from_secs(60));

    // SymbolCache avoids re-resolving the conid on every run.
    let cache = bezant::SymbolCache::new(client.clone());
    let conid = cache.conid_for(&symbol).await?;
    println!("{symbol} → conid {conid}");

    let mut ws = bezant::WsClient::connect(&client).await?;
    ws.subscribe_market_data(conid, &bezant::MarketDataFields::default_l1())
        .await?;

    // Read messages until the socket closes or we get SIGINT.
    tokio::select! {
        _ = tokio::signal::ctrl_c() => {
            println!("\nctrl-c, closing socket");
        }
        res = async {
            while let Some(msg) = ws.next_message().await? {
                match msg {
                    bezant::WsMessage::MarketData { conid, payload } => {
                        println!("tick conid={conid} {payload}");
                    }
                    bezant::WsMessage::Heartbeat => {}
                    other => println!("ws: {other:?}"),
                }
            }
            Ok::<_, bezant::Error>(())
        } => {
            res?;
        }
    }
    Ok(())
}
