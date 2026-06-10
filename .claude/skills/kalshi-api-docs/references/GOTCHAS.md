# Gotchas

## WebSocket Connections Require Auth

This applies to WebSockets.

- Even channels labeled `public` still require an authenticated WebSocket handshake.
- In WebSocket docs, `public` means public market data, not anonymous access.

## Multiple URLs Exist

The external API hosts were introduced on 2026-05-07. Use these:

- Live REST root: `https://external-api.kalshi.com/trade-api/v2`
- Live WebSocket root: `wss://external-api-ws.kalshi.com/trade-api/ws/v2`
- Demo REST root: `https://external-api.demo.kalshi.co/trade-api/v2`
- Demo WebSocket root: `wss://external-api-ws.demo.kalshi.co/trade-api/ws/v2`
- Demo credentials are separate from live credentials.

The old hosts (`api.elections.kalshi.com`, `demo-api.kalshi.co`) are no longer used as of 0.5.0.
