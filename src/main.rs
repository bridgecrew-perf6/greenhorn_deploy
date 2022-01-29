use std::{env, path::PathBuf};
use tracing_subscriber::{filter, prelude::*};

mod pull;
mod server;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    let filter = filter::Targets::new()
        // Enable the `INFO` level for anything in `my_crate`
        .with_target("greenhorn_deploy", tracing::Level::DEBUG)
        .with_target("axum", tracing::Level::INFO)
        .with_target("tower_http", tracing::Level::INFO);

    let registry = tracing_subscriber::registry();
    match tracing_journald::layer() {
        Ok(subscriber) => {
            registry.with(subscriber.with_filter(filter)).init();
        }
        Err(e) => {
            registry.init();
            tracing::error!("Couldn't connect to journald: {}", e);
        }
    }

    let path = PathBuf::from(
        args.get(1)
            .expect("No path parameter given\nUsage: greenhorn_deploy [path_to_repo]"),
    );

    let signature =
        env::var("GREENHORN_DEPLOY_SIGNATURE").expect("GREENHORN_DEPLOY_SIGNATURE not set");

    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 8765));

    tracing::debug!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(server::app(path, signature).into_make_service())
        .await
        .unwrap();
}
