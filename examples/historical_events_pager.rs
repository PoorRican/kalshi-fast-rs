/// Paginate through settled events with custom termination logic.
///
/// Uses CursorPager for page-by-page control, stopping when all events in a
/// batch closed before December 2025. `EventData` carries no close timestamp
/// itself, so we request nested markets and check each market's
/// `close_time` client-side.
use chrono::{DateTime, TimeZone, Utc};
use kalshi_fast::{EventStatus, GetEventsParams, KalshiEnvironment, KalshiRestClient};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = KalshiRestClient::new(KalshiEnvironment::production());

    // December 1, 2025 00:00:00 UTC
    let cutoff = Utc.with_ymd_and_hms(2025, 12, 1, 0, 0, 0).unwrap();

    let mut pager = client.events_pager(GetEventsParams {
        status: Some(EventStatus::Settled),
        limit: Some(200),
        with_nested_markets: Some(true),
        ..Default::default()
    });

    while let Some(events) = pager.next_page().await? {
        let event_close_time = |event: &kalshi_fast::EventData| -> Option<DateTime<Utc>> {
            event
                .markets
                .as_ref()?
                .iter()
                .filter_map(|m| m.close_time.as_deref())
                .filter_map(|t| DateTime::parse_from_rfc3339(t).ok())
                .map(|t| t.with_timezone(&Utc))
                .max()
        };

        let all_before_cutoff = events
            .iter()
            .all(|e| event_close_time(e).is_some_and(|t| t < cutoff));

        for event in &events {
            println!(
                "{} | close_time: {:?}",
                event.event_ticker,
                event_close_time(event)
            );
        }

        if all_before_cutoff {
            println!("All events closed before Dec 2025 - stopping");
            break;
        }
    }

    Ok(())
}
