#![cfg(feature = "live-tests")]

mod common;

use futures::StreamExt;
use kalshi_fast::{
    EventStatus, GetEventForecastPercentileHistoryParams, GetEventsParams,
    GetMultivariateEventsParams,
};
use std::collections::HashSet;

#[tokio::test]
async fn test_get_event_forecast_percentile_history() {
    let client = common::demo_client();
    let (series_ticker, event_ticker) = common::pick_event_with_series(&client).await;

    let params = GetEventForecastPercentileHistoryParams {
        percentiles: vec![25, 50, 75],
        start_ts: common::now_minus_days(14),
        end_ts: common::now_minus_days(1),
        period_interval: 1440,
    };

    let resp = tokio::time::timeout(
        common::TEST_TIMEOUT,
        client.get_event_forecast_percentile_history(&series_ticker, &event_ticker, params.clone()),
    )
    .await
    .expect("timeout")
    .expect("request failed");

    if resp.forecast_history.is_empty() {
        eprintln!("skipping: demo has no forecast history for {series_ticker}/{event_ticker}");
        return;
    }

    let mut prev_ts = i64::MIN;
    for point in &resp.forecast_history {
        assert_eq!(point.event_ticker, event_ticker);
        assert!(
            point.end_period_ts >= params.start_ts && point.end_period_ts <= params.end_ts,
            "forecast ts {} outside [{}, {}]",
            point.end_period_ts,
            params.start_ts,
            params.end_ts
        );
        assert!(
            point.end_period_ts > prev_ts,
            "forecast timestamps must be strictly ascending: {} <= {}",
            point.end_period_ts,
            prev_ts
        );
        prev_ts = point.end_period_ts;

        let returned: HashSet<i32> = point
            .percentile_points
            .iter()
            .map(|p| p.percentile)
            .collect();
        for requested in [25, 50, 75] {
            assert!(
                returned.contains(&requested),
                "requested percentile {} missing from response: {:?}",
                requested,
                returned
            );
        }
    }
}

#[tokio::test]
async fn test_events_pager_disjoint_pages() {
    let client = common::demo_client();

    let mut pager = client.events_pager(GetEventsParams {
        limit: Some(5),
        ..Default::default()
    });

    let page1 = tokio::time::timeout(common::TEST_TIMEOUT, pager.next_page())
        .await
        .expect("timeout")
        .expect("page 1 request failed")
        .expect("expected a first page");

    if pager.is_done() {
        eprintln!("skipping: demo returned fewer than 2 pages of events");
        return;
    }

    let page2 = tokio::time::timeout(common::TEST_TIMEOUT, pager.next_page())
        .await
        .expect("timeout")
        .expect("page 2 request failed")
        .expect("expected a second page");

    let set1: HashSet<String> = page1.iter().map(|e| e.event_ticker.clone()).collect();
    let set2: HashSet<String> = page2.iter().map(|e| e.event_ticker.clone()).collect();
    assert!(
        set1.is_disjoint(&set2),
        "pages must be disjoint; overlap = {:?}",
        set1.intersection(&set2).collect::<Vec<_>>()
    );
}

#[tokio::test]
async fn test_stream_events_bounded() {
    let client = common::demo_client();

    let stream = client.stream_events(
        GetEventsParams {
            limit: Some(5),
            ..Default::default()
        },
        Some(10),
    );
    tokio::pin!(stream);

    let mut tickers = HashSet::new();
    while let Some(item) = tokio::time::timeout(common::TEST_TIMEOUT, stream.next())
        .await
        .expect("timeout awaiting stream item")
    {
        let event = item.expect("stream yielded error");
        assert!(
            tickers.insert(event.event_ticker.clone()),
            "duplicate: {}",
            event.event_ticker
        );
    }

    assert!(
        tickers.len() <= 10,
        "stream exceeded max_items: {}",
        tickers.len()
    );
}

#[tokio::test]
async fn test_stream_multivariate_events_bounded() {
    let client = common::demo_client();

    let stream = client.stream_multivariate_events(
        GetMultivariateEventsParams {
            limit: Some(5),
            ..Default::default()
        },
        Some(5),
    );
    tokio::pin!(stream);

    let mut count = 0usize;
    while let Some(item) = tokio::time::timeout(common::TEST_TIMEOUT, stream.next())
        .await
        .expect("timeout awaiting stream item")
    {
        let event = item.expect("stream yielded error");
        assert!(!event.event_ticker.is_empty());
        count += 1;
    }

    assert!(count <= 5, "stream exceeded max_items: {count}");
}

#[tokio::test]
async fn test_get_event_by_ticker_with_nested_markets() {
    let client = common::demo_client();

    let list_resp = tokio::time::timeout(
        common::TEST_TIMEOUT,
        client.get_events(GetEventsParams {
            limit: Some(1),
            status: Some(EventStatus::Open),
            ..Default::default()
        }),
    )
    .await
    .expect("timeout")
    .expect("request failed");

    let Some(list_event) = list_resp.events.into_iter().next() else {
        eprintln!("skipping: no open events on demo");
        return;
    };

    let resp = tokio::time::timeout(
        common::TEST_TIMEOUT,
        client.get_event(&list_event.event_ticker, Some(true)),
    )
    .await
    .expect("timeout")
    .expect("request failed");

    assert_eq!(resp.event.event_ticker, list_event.event_ticker);

    let nested = resp
        .event
        .markets
        .as_ref()
        .expect("with_nested_markets=true should populate markets");
    assert!(!nested.is_empty(), "nested markets should be non-empty");
    for market in nested {
        assert!(!market.ticker.is_empty());
    }
}

#[tokio::test]
async fn test_get_events_with_milestones_flag() {
    let client = common::demo_client();

    let resp = tokio::time::timeout(
        common::TEST_TIMEOUT,
        client.get_events(GetEventsParams {
            limit: Some(10),
            with_milestones: Some(true),
            ..Default::default()
        }),
    )
    .await
    .expect("timeout")
    .expect("request failed");

    // `with_milestones=true` returns related milestones as a response-level
    // sibling array (`GetEventsResponse.milestones`), not nested per event.
    for milestone in &resp.milestones {
        // Each milestone should carry at least one identifying field.
        assert!(
            milestone.id.is_some() || milestone.title.is_some() || milestone.name.is_some(),
            "milestone lacks any identifier: {:?}",
            milestone
        );
    }
}
