# kalshi-fast-rs

Rust Kalshi REST/WebSocket client library for algorithmic trading; maintain fully tested, feature-complete behavior against the published API.

## Read first

Read workspace-root `AGENTS.md` for shared policy; use `kairos-context/AGENTS.md` for Kairos cross-repo/vendor contracts when relevant. For API or transport behavior, consult the applicable Kalshi published source; upstream refreshes must re-fetch all four under the repo-local refresh workflow:
- `https://docs.kalshi.com/llms.txt`
- `https://docs.kalshi.com/changelog/rss.xml`
- `https://docs.kalshi.com/openapi.yaml`
- `https://docs.kalshi.com/asyncapi.yaml`
Read `VERSIONING.md` for public-API compatibility, upstream-alignment, or release-metadata changes; `.codex/skills/kalshi-refresh/SKILL.md` owns the upstream refresh workflow.
Load `market-data-vendors` for vendor behavior and `rust-engineering` for Rust implementation changes.

## Performance and credentials

- Minimize hot-path allocations; prefer borrowing over avoidable cloning, zero-copy WebSocket parsing where possible, and abstractions without runtime overhead. Precompute reusable work, including reuse of the signing key.
- `test_key.pem` is the demo private key; `.env.test` owns its matching demo key ID/settings. `.kalshi_api_key.pem` is live; `.env` owns its matching live key ID/settings. Never expose credential contents or confuse demo and production.
- Credential access is not permission to place live orders; live-money actions require explicit owner authorization.

## Checks

- Offline CI checks (`.github/workflows/rust.yml`): `cargo fmt --all --check`, `cargo clippy --all-targets`, `cargo build --verbose`, and `cargo test --all-targets --verbose`; also run `cargo clippy --all-targets --features timed-reader` and `cargo test --all-targets --features timed-reader --verbose`. These do not enable the live-network features.
- `live-tests` enables network integration tests. The order and order-group lifecycle suites mutate demo state using `tests/common/mod.rs` and `.env.test`; keep their endpoint and credential settings on demo rather than substituting the live files above.
- `live-prod-tests` includes `live-tests` and adds production WebSocket probes. Thus `--all-features` is not an offline gate. Select the needed feature/test targets under `.codex/skills/kalshi-refresh/SKILL.md` and the shared authorization policy.

## Shared policy

Workspace-root `AGENTS.md` owns shared authorization, knowledge placement, skill delivery, memory, and evidence-link rules.
