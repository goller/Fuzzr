# Fuzzr Web Publishing

Analogous to a CMS (not static site), but self-hosted on-prem, using:

- CloudFlare Argo tunnel daemon
- CommonMark & YFM rendering

## 0.1 - Basic Publishing

- [x] Text entry
- [x] ~~Save text to existing IPFS embed backend (may have to fallback to sled backend feature flag experimentation if ipfs-embed-rs retrieval continues to be a roadblock)~~
- [ ] Save text in local Git site repo, similar to how static sites do.
- [ ] Warp server
- [ ] Configure cloudflared Argo tunnel daemon
    - [https://github.com/cloudflare/cloudflared](https://github.com/cloudflare/cloudflared)
    - [https://developers.cloudflare.com/argo-tunnel/](https://developers.cloudflare.com/argo-tunnel/)

## 0.2 - Web UI

- [ ] Iced web UI
- [ ] Websockets
- [ ] CBOR packets
- [ ] Clientside CommonMark rendering
- [ ] Static file server
- [ ] Usage and Setup Documentation for Publishers

## 0.2.1 - Monochrome Design

- [ ] Styles
    - [ ] Monochrome (two colors)
    - [ ] Thin lines (2px)
    - [ ] Blunt corners (2px rounded)
- [ ] Themes
    - [ ] Dark theme
    - [ ] Light theme
    - [ ] Serverside detection
    - [ ] Fallback clientside detection
    - [ ] Button for dark mode setting
    - [ ] Dark mode setting persistence
    - [ ] Fallback to default (dark theme)

## 0.3 - Crypto monetization v1 - Fuzzr (website, crypto)

- [ ] Miner config
    - [ ] Communicate with locally running xmrig HTTP API
- [ ] Wallet config
    - [ ] Communicate with locally running monerod JSON-RPC API
- [ ] Mining pool verification
    - [ ] reqwest to pool endpoint

## 0.4 - User-generated content

- [ ] Token / Cookie client authentication
- [ ] Voting (in MH)
    - [ ] (1 MH can usually be reached within 5-20 minutes of xmrig mining, dependent on hardware)
- [ ] Comments

## 0.5 - P2P

- [ ] P2P IPFS (for more than just local storage backend)
- [ ] Publisher notifications
- [ ] Subscriptions

## Notes

- Look into CloudFlare cache for offline / maintenance operation
- Look into CloudFlare HTTP/3 to HTTP/2 and HTTP 1.1 backward compatibility

## Ongoing technical decisions

- Decide on data backend: ipfs (embedded dht), sled (embedded LSM-tree k/v store), pallet (embedded searchable document store, unmaintained, design flaws, tantivy, but promising)
    - Start with existing ipfs-embed-rs, but it's currently switching to a SQLite backend to better support object GC, which may or may not be desirable, depending.
- Future HTTP/3 support: h3 -> hyper -> warp