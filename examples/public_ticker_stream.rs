use anyhow;
use kalshi::{KalshiEnvironment, KalshiWsClient, WsChannel};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let env = KalshiEnvironment::demo();
    let mut ws = KalshiWsClient::connect(env).await?;

    ws.subscribe(vec![WsChannel::Ticker], None).await?;

    loop {
        let msg = ws.next_envelope().await?;
        match msg.msg_type.as_str() {
            "ticker" => {
                let ticker = msg.parse_ticker()?;
                println!("type=ticker id={:?} market={} price={}", msg.id, ticker.market_ticker, ticker.price);
            }
            other => {
                println!("type={} id={:?} msg={:?}", other, msg.id, msg.msg_raw());
            }
        }
    }
}
