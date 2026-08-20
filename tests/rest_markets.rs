#![cfg(feature = "live-tests")]

mod common;

use futures::StreamExt;
use kalshi_fast::{
    GetHistoricalMarketsParams, GetMarketCandlesticksHistoricalParams, GetMarketOrderbooksParams,
    GetMarketsParams, MarketStatus, MarketStatusQuery, MveFilter,
};
use std::collections::HashSet;

#[tokio::test]
async fn test_get_historical_market() {
    let client = common::demo_client();
    let settled = common::pick_settled_market(&client).await;
    let ticker = settled.ticker.clone();

    let resp = tokio::time::timeout(common::TEST_TIMEOUT, client.get_historical_market(&ticker))
        .await
        .expect("timeout")
        .expect("request failed");

    assert_eq!(resp.market.ticker, ticker);
    assert!(
        matches!(
            resp.market.status,
            Some(
                MarketStatus::Closed
                    | MarketStatus::Finalized
                    | MarketStatus::Determined
                    | MarketStatus::Disputed
                    | MarketStatus::Amended
            )
        ),
        "expected historical market to be closed/settled, got {:?}",
        resp.market.status
    );
    assert!(
        resp.market
            .event_ticker
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    );
    assert!(
        resp.market
            .series_ticker
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    );
}

#[tokio::test]
async fn test_get_historical_markets() {
    let client = common::demo_client();
    let resp = tokio::time::timeout(
        common::TEST_TIMEOUT,
        client.get_historical_markets(GetHistoricalMarketsParams {
            limit: Some(10),
            ..Default::default()
        }),
    )
    .await
    .expect("timeout")
    .expect("request failed");

    assert!(resp.markets.len() <= 10);

    let mut seen = HashSet::new();
    for m in &resp.markets {
        assert!(!m.ticker.is_empty());
        assert!(
            seen.insert(m.ticker.clone()),
            "duplicate ticker: {}",
            m.ticker
        );
    }
}

#[tokio::test]
async fn test_get_market_orderbooks_batch() {
    let client = common::demo_client();
    let markets_resp = tokio::time::timeout(
        common::TEST_TIMEOUT,
        client.get_markets(GetMarketsParams {
            limit: Some(3),
            status: Some(MarketStatusQuery::Open),
            ..Default::default()
        }),
    )
    .await
    .expect("timeout")
    .expect("request failed");

    if markets_resp.markets.len() < 2 {
        eprintln!("skipping: demo returned fewer than 2 open markets");
        return;
    }

    let tickers: Vec<String> = markets_resp
        .markets
        .iter()
        .map(|m| m.ticker.clone())
        .collect();
    let requested: HashSet<String> = tickers.iter().cloned().collect();

    let resp = tokio::time::timeout(
        common::TEST_TIMEOUT,
        client.get_market_orderbooks(GetMarketOrderbooksParams {
            tickers: tickers.clone(),
        }),
    )
    .await
    .expect("timeout")
    .expect("request failed");

    assert_eq!(resp.orderbooks.len(), tickers.len());

    for ob in &resp.orderbooks {
        assert!(
            requested.contains(&ob.ticker),
            "response ticker {} not in request set",
            ob.ticker
        );

        for (price, _qty) in &ob.orderbook_fp.yes_dollars {
            let p: f64 = price.parse().expect("yes price parses as f64");
            assert!(p > 0.0 && p < 1.0, "yes price out of range: {}", price);
        }
        for (price, _qty) in &ob.orderbook_fp.no_dollars {
            let p: f64 = price.parse().expect("no price parses as f64");
            assert!(p > 0.0 && p < 1.0, "no price out of range: {}", price);
        }
    }
}

#[tokio::test]
async fn test_get_historical_market_candlesticks() {
    let client = common::demo_client();
    let settled = common::pick_settled_market(&client).await;

    let end_ts = common::now_minus_days(1);
    let start_ts = common::now_minus_days(30);
    let params = GetMarketCandlesticksHistoricalParams {
        start_ts,
        end_ts,
        period_interval: 60,
    };

    let resp = tokio::time::timeout(
        common::TEST_TIMEOUT,
        client.get_historical_market_candlesticks(&settled.ticker, params),
    )
    .await
    .expect("timeout")
    .expect("request failed");

    assert_eq!(resp.ticker, settled.ticker);

    if resp.candlesticks.is_empty() {
        eprintln!(
            "skipping candlestick assertions: no historical candles for {}",
            settled.ticker
        );
        return;
    }

    let mut prev_ts = i64::MIN;
    for candle in &resp.candlesticks {
        assert!(
            candle.end_period_ts >= start_ts && candle.end_period_ts <= end_ts,
            "candle ts {} outside [{}, {}]",
            candle.end_period_ts,
            start_ts,
            end_ts
        );
        assert!(
            candle.end_period_ts > prev_ts,
            "candle timestamps must be strictly ascending: {} <= {}",
            candle.end_period_ts,
            prev_ts
        );
        prev_ts = candle.end_period_ts;

        let volume: f64 = candle.volume.parse().expect("volume parses as f64");
        assert!(volume >= 0.0, "non-negative volume: {}", candle.volume);
    }
}

#[tokio::test]
async fn test_markets_pager_disjoint_pages() {
    let client = common::demo_client();

    let mut pager = client.markets_pager(GetMarketsParams {
        limit: Some(5),
        ..Default::default()
    });

    let page1 = tokio::time::timeout(common::TEST_TIMEOUT, pager.next_page())
        .await
        .expect("timeout")
        .expect("page 1 request failed")
        .expect("expected a first page");

    if pager.is_done() {
        eprintln!("skipping: demo returned fewer than 2 pages of markets");
        return;
    }

    assert!(pager.current_cursor().is_some());

    let page2 = tokio::time::timeout(common::TEST_TIMEOUT, pager.next_page())
        .await
        .expect("timeout")
        .expect("page 2 request failed")
        .expect("expected a second page");

    let set1: HashSet<String> = page1.iter().map(|m| m.ticker.clone()).collect();
    let set2: HashSet<String> = page2.iter().map(|m| m.ticker.clone()).collect();
    assert!(
        set1.is_disjoint(&set2),
        "pages must be disjoint; overlap = {:?}",
        set1.intersection(&set2).collect::<Vec<_>>()
    );
}

#[tokio::test]
async fn test_stream_markets_bounded() {
    let client = common::demo_client();

    let stream = client.stream_markets(
        GetMarketsParams {
            limit: Some(5),
            ..Default::default()
        },
        Some(12),
    );
    tokio::pin!(stream);

    let mut tickers = HashSet::new();
    while let Some(item) = tokio::time::timeout(common::TEST_TIMEOUT, stream.next())
        .await
        .expect("timeout awaiting stream item")
    {
        let market = item.expect("stream yielded error");
        assert!(
            tickers.insert(market.ticker.clone()),
            "duplicate: {}",
            market.ticker
        );
    }

    assert!(
        tickers.len() <= 12,
        "stream exceeded max_items: {}",
        tickers.len()
    );
}

#[tokio::test]
async fn test_get_markets_cross_consistency() {
    let client = common::demo_client();

    let list_resp = tokio::time::timeout(
        common::TEST_TIMEOUT,
        client.get_markets(GetMarketsParams {
            limit: Some(1),
            status: Some(MarketStatusQuery::Open),
            ..Default::default()
        }),
    )
    .await
    .expect("timeout")
    .expect("request failed");

    let list_market = list_resp
        .markets
        .into_iter()
        .next()
        .expect("demo returned no open markets");

    let single_resp =
        tokio::time::timeout(common::TEST_TIMEOUT, client.get_market(&list_market.ticker))
            .await
            .expect("timeout")
            .expect("request failed");

    assert_eq!(single_resp.market.ticker, list_market.ticker);
    assert_eq!(single_resp.market.event_ticker, list_market.event_ticker);
    assert_eq!(single_resp.market.series_ticker, list_market.series_ticker);
}

#[tokio::test]
async fn test_get_markets_status_filter_correctness() {
    let client = common::demo_client();

    let resp = tokio::time::timeout(
        common::TEST_TIMEOUT,
        client.get_markets(GetMarketsParams {
            limit: Some(20),
            status: Some(MarketStatusQuery::Closed),
            ..Default::default()
        }),
    )
    .await
    .expect("timeout")
    .expect("request failed");

    if resp.markets.is_empty() {
        eprintln!("skipping: no closed markets on demo");
        return;
    }

    // MarketStatusQuery::Closed maps to lifecycle statuses Closed/Determined/Disputed/Amended.
    for market in &resp.markets {
        let status = market.status.expect("status present on filtered response");
        assert!(
            matches!(
                status,
                MarketStatus::Closed
                    | MarketStatus::Determined
                    | MarketStatus::Disputed
                    | MarketStatus::Amended
            ),
            "status filter returned unexpected status {:?} for {}",
            status,
            market.ticker
        );
    }
}

#[tokio::test]
async fn test_get_markets_mve_filter_only() {
    let client = common::demo_client();

    let resp = tokio::time::timeout(
        common::TEST_TIMEOUT,
        client.get_markets(GetMarketsParams {
            limit: Some(10),
            mve_filter: Some(MveFilter::Only),
            ..Default::default()
        }),
    )
    .await
    .expect("timeout")
    .expect("request failed");

    if resp.markets.is_empty() {
        eprintln!("skipping: no MVE markets on demo");
        return;
    }

    for market in &resp.markets {
        assert!(
            market
                .mve_collection_ticker
                .as_ref()
                .is_some_and(|v| !v.is_empty())
                || market
                    .mve_selected_legs
                    .as_ref()
                    .is_some_and(|v| !v.is_empty()),
            "mve_filter=only should return markets with mve metadata; got {:?}",
            market.ticker
        );
    }
}

#[tokio::test]
async fn test_get_markets_price_ranges_are_authoritative() {
    let client = common::demo_client();

    let resp = tokio::time::timeout(
        common::TEST_TIMEOUT,
        client.get_markets(GetMarketsParams {
            limit: Some(20),
            status: Some(MarketStatusQuery::Open),
            ..Default::default()
        }),
    )
    .await
    .expect("timeout")
    .expect("request failed");

    if resp.markets.is_empty() {
        eprintln!("skipping: no open markets on demo");
        return;
    }

    for market in &resp.markets {
        let Some(ranges) = market.price_ranges.as_ref() else {
            continue;
        };
        assert!(
            !ranges.is_empty(),
            "price_ranges present but empty for {}",
            market.ticker
        );

        let mut prev_end: Option<f64> = None;
        for range in ranges {
            let start: f64 = range.start.parse().expect("range start parses as f64");
            let end: f64 = range.end.parse().expect("range end parses as f64");
            let step: f64 = range.step.parse().expect("range step parses as f64");

            assert!(
                start < end,
                "price range must be ascending for {}: {} .. {}",
                market.ticker,
                range.start,
                range.end
            );
            assert!(
                step > 0.0,
                "price range step must be positive for {}: {}",
                market.ticker,
                range.step
            );
            if let Some(prev) = prev_end {
                assert!(
                    start >= prev,
                    "price ranges must not overlap for {}: {} < {}",
                    market.ticker,
                    range.start,
                    prev
                );
            }
            prev_end = Some(end);
        }

        // The structure label is informational only; it must never be used to
        // derive tick sizes, so we only assert it is non-empty when present.
        if let Some(structure) = market.price_level_structure.as_ref() {
            assert!(
                !structure.is_empty(),
                "price_level_structure present but empty for {}",
                market.ticker
            );
        }
    }
}

/// Diagnostic: audit which `Option<_>` fields on `Market` are `None` across a
/// batch of live demo markets. Intended to be run with `-- --nocapture` to
/// gather evidence before any breaking type tightening.
#[tokio::test]
async fn audit_market_option_fields() {
    let client = common::demo_client();

    let markets = tokio::time::timeout(
        common::TEST_TIMEOUT,
        client.get_markets(GetMarketsParams {
            limit: Some(50),
            ..Default::default()
        }),
    )
    .await
    .expect("timeout")
    .expect("request failed");

    let n = markets.markets.len();
    if n == 0 {
        eprintln!("audit: no markets returned");
        return;
    }

    let count_none = |iter: Box<dyn Iterator<Item = bool> + '_>| iter.filter(|b| *b).count();

    let event_ticker_none = count_none(Box::new(
        markets.markets.iter().map(|m| m.event_ticker.is_none()),
    ));
    let series_ticker_none = count_none(Box::new(
        markets.markets.iter().map(|m| m.series_ticker.is_none()),
    ));
    let market_type_none = count_none(Box::new(
        markets.markets.iter().map(|m| m.market_type.is_none()),
    ));
    let rules_primary_none = count_none(Box::new(
        markets.markets.iter().map(|m| m.rules_primary.is_none()),
    ));
    let status_none = count_none(Box::new(markets.markets.iter().map(|m| m.status.is_none())));
    let title_none = count_none(Box::new(markets.markets.iter().map(|m| m.title.is_none())));
    let can_trade_none = count_none(Box::new(
        markets.markets.iter().map(|m| m.can_trade.is_none()),
    ));

    eprintln!("--- Market Option<_> audit (n = {n}) ---");
    eprintln!("event_ticker  None in {event_ticker_none}/{n}");
    eprintln!("series_ticker None in {series_ticker_none}/{n}");
    eprintln!("market_type   None in {market_type_none}/{n}");
    eprintln!("rules_primary None in {rules_primary_none}/{n}");
    eprintln!("status        None in {status_none}/{n}");
    eprintln!("title         None in {title_none}/{n}");
    eprintln!("can_trade     None in {can_trade_none}/{n}");
}
