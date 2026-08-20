#![cfg(feature = "live-tests")]

mod common;

use kalshi_fast::GetSeriesListParams;

#[tokio::test]
async fn test_series_cross_consistency() {
    let client = common::demo_client();

    let list_resp = tokio::time::timeout(
        common::TEST_TIMEOUT,
        client.get_series_list(GetSeriesListParams::default()),
    )
    .await
    .expect("timeout")
    .expect("request failed");

    let first = list_resp
        .series
        .into_iter()
        .next()
        .expect("demo returned no series");

    let detail_resp = tokio::time::timeout(common::TEST_TIMEOUT, client.get_series(&first.ticker))
        .await
        .expect("timeout")
        .expect("request failed");

    assert_eq!(detail_resp.series.ticker, first.ticker);
    assert!(
        detail_resp.series.frequency.is_some(),
        "series {} missing frequency field",
        first.ticker
    );
    assert_eq!(detail_resp.series.title, first.title);
    // `exchange_index` was added to series responses on 2026-07-30; when the
    // list response carries it, the detail response must agree.
    if first.exchange_index.is_some() {
        assert_eq!(
            detail_resp.series.exchange_index, first.exchange_index,
            "series {} exchange_index differs between list and detail",
            first.ticker
        );
    }
}
