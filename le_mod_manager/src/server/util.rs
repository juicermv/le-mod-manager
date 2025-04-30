use std::net::TcpListener;
use std::ops::Range;
use anyhow::anyhow;
use dioxus::logger::tracing;

pub fn try_pick_random_port(range: Range<u16>) -> anyhow::Result<u16> {
    for port in range {
        if let Ok(listener) = TcpListener::bind(("127.0.0.1", port)) {
            drop(listener);
            return Ok(port);
        }
    }
    Err(anyhow!("No port available."))
}