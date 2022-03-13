use anyhow::{bail, Result};
use std::net::IpAddr;
use tokio::{sync::oneshot, task::JoinHandle};

// Module containing our api paths (routes.rs)
mod routes;

// Api struct to hold everything we need for keeping track of the listening socket
pub struct Api {
    pub ip: IpAddr,
    pub port: u16,
    handle: Option<JoinHandle<()>>,
    shutdown_sender: Option<oneshot::Sender<()>>,
}

// Function implementations for "Api" struct
impl Api {
    pub fn new(ip: IpAddr, port: u16) -> Result<Self> {
        // Oneshot channel to tell the webserver when to exit (necessary since webserver will be in a different thread)
        let (shutdown_sender, shutdown_receiver) = oneshot::channel();

        // Initialise Api struct
        let mut result = Self {
            ip,
            port,
            handle: None,
            shutdown_sender: Some(shutdown_sender),
        };

        // Spawn webserver thread and save join handle to Api struct
        result.handle = Some(tokio::spawn(async move {
            Self::run(ip, port, shutdown_receiver).await;
        }));

        // return
        Ok(result)
    }

    // Function to instruct webserver to exit
    pub async fn stop(&mut self) -> Result<()> {
        // Get channel sender out of Api struct and send empty object
        if let Some(sender) = self.shutdown_sender.take() {
            if sender.send(()).is_err() {
                bail!("Failed to stop web server!")
            }
        }

        // Wait for join handle to exit
        if let Err(msg) = self.handle.take().unwrap().await {
            bail!(msg);
        }

        Ok(())
    }

    // Function to govern webserver thread
    async fn run(ip: IpAddr, port: u16, shutdown_receiver: oneshot::Receiver<()>) {
        // Get routes from routes.rs
        let routes = routes::get_routes();

        let (_, server) = warp::serve(routes).bind_with_graceful_shutdown((ip, port), async {
            // Wait for shutdown signal
            shutdown_receiver.await.ok();
        });

        server.await;
    }
}
