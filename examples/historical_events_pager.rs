/// Paginate events using the documented settled-status and minimum-close filters.
///
/// Event objects have no aggregate close timestamp. Apply the lower close bound
/// on the server rather than inferring a timestamp or pagination ordering.
use chrono::{TimeZone, Utc};
use kalshi_fast::{EventStatus, GetEventsParams, KalshiEnvironment, KalshiRestClient};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = KalshiRestClient::new(KalshiEnvironment::production());

    // December 1, 2025 00:00:00 UTC
    let cutoff_ts = Utc
        .with_ymd_and_hms(2025, 12, 1, 0, 0, 0)
        .unwrap()
        .timestamp();

    let mut pager = client.events_pager(GetEventsParams {
        status: Some(EventStatus::Settled),
        limit: Some(200),
        min_close_ts: Some(cutoff_ts),
        ..Default::default()
    });

    while let Some(events) = pager.next_page().await? {
        for event in &events {
            println!("{} | series: {:?}", event.event_ticker, event.series_ticker);
        }
    }

    Ok(())
}
