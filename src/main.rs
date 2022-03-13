#![forbid(unsafe_code)]

// Generic result type for convenience
use anyhow::Result;
// Faster malloc implementation that can be statically linked
use mimalloc::MiMalloc;
// Ip address primitives
use std::net::{IpAddr, Ipv4Addr};
// Async runtime
use tokio::{
    runtime::Builder,
    signal::unix::{signal, SignalKind},
};

// Load api/mod.rs
pub mod api;

// Use mimalloc as global allocator
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

// Signal handler for messages sent to container
async fn wait_for_signal() {
    // Register listeners for process signals
    let mut hangup = signal(SignalKind::hangup()).expect("Unable to register signal: SIGHUP");
    let mut int = signal(SignalKind::interrupt()).expect("Unable to register signal: SIGINT");
    let mut quit = signal(SignalKind::quit()).expect("Unable to register signal: SIGQUIT");
    let mut alarm = signal(SignalKind::alarm()).expect("Unable to register signal: SIGALRM");
    let mut term = signal(SignalKind::terminate()).expect("Unable to register signal: SIGTERM");

    // Pick the first signal to trigger
    let sig = tokio::select! {
        _ = hangup.recv() => "hangup",
        _ = int.recv() => "interrupt",
        _ = quit.recv() => "quit",
        _ = alarm.recv() => "alarm",
        _ = term.recv() => "term",
        else => "no signals"
    };

    println!("Signal received: {}", sig);
}

// Async entry point
async fn start() -> Result<()> {
    // Initialise socket information
    let ip = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0));
    let port = 8080;

    // Launch api
    let mut api = api::Api::new(ip, port).unwrap();

    println!("Running on {}:{}", ip, port);
    println!("Waiting for signal...");

    // Wait until process signal received
    wait_for_signal().await;

    // Gracefully exit api
    api.stop().await
}

// Program entry point
fn main() {
    // Initialise Tokio runtime
    let rt = Builder::new_multi_thread()
        .enable_all()
        .worker_threads(num_cpus::get())
        .thread_name("tokio")
        .max_blocking_threads(4096)
        .build()
        .unwrap();

    // Launch Tokio runtime via start() function
    rt.block_on(async { start().await.ok() });
}
