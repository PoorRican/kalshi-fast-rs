/// Paginate through settled events with custom termination logic.
///
/// Uses CursorPager for page-by-page control, stopping when all markets in a
/// batch closed before December 2025. `EventData` carries no close timestamp
/// of its own (removed from the OpenAPI schema), so nested markets are
/// requested and their `close_time` is checked client-side.
use chrono::{DateTime, TimeZone, Utc};
use kalshi_fast::{EventStatus, GetEventsParams, KalshiEnvironment, KalshiRestClient};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = KalshiRestClient::new(KalshiEnvironment::production());

    // December 1, 2025 00:00:00 UTC
    let cutoff = Utc.with_ymd_and_hms(2025, 12, 1, 0, 0, 0).unwrap();

    let mut pager = client.events_pager(GetEventsParams {
        status: Some(EventStatus::Settled),
        with_nested_markets: Some(true),
        limit: Some(200),
        ..Default::default()
    });

    while let Some(events) = pager.next_page().await? {
        let all_before_cutoff = events.iter().all(|e| {
            e.markets.as_deref().unwrap_or_default().iter().all(|m| {
                m.close_time
                    .as_deref()
                    .and_then(|t| DateTime::parse_from_rfc3339(t).ok())
                    .is_some_and(|ts| ts < cutoff)
            })
        });

        for event in &events {
            let close_times: Vec<&str> = event
                .markets
                .as_deref()
                .unwrap_or_default()
                .iter()
                .filter_map(|m| m.close_time.as_deref())
                .collect();
            println!("{} | close_time: {:?}", event.event_ticker, close_times);
        }

        if all_before_cutoff {
            println!("All events closed before Dec 2025 - stopping");
            break;
        }
    }

    Ok(())
}
