# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.0] - 2026-03-05

### Added

- Added `MarketStatusConversionError` for strict lifecycle/query status conversions.
- Added best-effort `From` conversions between lifecycle `MarketStatus` and query `MarketStatusQuery`.
- Added strict `TryFrom<&...>` conversions for exact one-to-one status mapping.
- Added/expanded parsing tests for status serialization and conversion behavior.
- Added `KalshiError::Parse` with parse context, human-readable reason, raw payload bytes, and optional serde source error.
- Added public parse accessors on `KalshiError`: `parse_context()`, `parse_error_reason()`, and `parse_raw_bytes()`.
- Added regression tests covering REST and WebSocket parse failures to verify reason text and raw-byte preservation.

### Changed

- Renamed query enum `MarketStatus` to `MarketStatusQuery`.
- Renamed REST market lifecycle enum `MarketState` to `MarketStatus`.
- Updated `GetMarketsParams.status` to use `Option<MarketStatusQuery>`.
- Updated `Market.status` to use `Option<MarketStatus>`.
- Updated examples, tests, and REST module docs to use the new names.
- REST success-response decoding now returns `KalshiError::Parse` (with raw bytes) instead of a plain serde JSON error.
- WebSocket envelope/message parsing now returns `KalshiError::Parse` with clearer parse-failure context and preserved raw payload bytes.

### Breaking

- Removed old `MarketState` and old query `MarketStatus` names without aliases.
- Downstream consumers must update imports and enum references to the new names.
- Added a new `KalshiError` enum variant (`Parse`); downstream exhaustive `match` statements over `KalshiError` must handle this variant.
