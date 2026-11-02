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

    let detail_resp =
        tokio::time::timeout(common::TEST_TIMEOUT, client.get_series(&first.ticker, None))
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
}
