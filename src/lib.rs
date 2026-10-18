//! `serve-epoll` — Handl stdlib port.

/// Checkout root of this repo (build-time).
pub const ROOT: &str = env!("CARGO_MANIFEST_DIR");

/// `index.hnl`.
pub const SERVE_EPOLL_HNL: &str = include_str!("index.hnl");
