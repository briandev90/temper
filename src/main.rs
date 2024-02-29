use std::{env, sync::Arc};

use dashmap::DashMap;

use enso_temper::{
    config::config,
    errors::handle_rejection,
    simulate_routes,
    SharedSimulationState,
};
use warp::{http::StatusCode, Filter};

#[tokio::main]
async fn main() {
    if env::var_os("RUST_LOG").is_none() {
        // Set `RUST_LOG=ts::api=debug` to see debug logs, this only shows access logs.
        env::set_var("RUST_LOG", "ts::api=info");
    }
    pretty_env_logger::init();

    let config = config();
    let port = config.port;

    let shared_state = Arc::new(SharedSimulationState {
        evms: Arc::new(DashMap::new()),
    });

    let api_base = warp::path("api").and(warp::path("v1"));
    let api_routes = api_base
        .and(simulate_routes(config, shared_state))
        .recover(handle_rejection)
        .with(warp::log("ts::api"));

    let health_route = warp::path!("health")
        .map(|| StatusCode::OK);

    let routes = health_route.or(api_routes);

    log::info!(
        target: "ts::api",
        "Starting server on port {port}"
    );
    warp::serve(routes).run(([0, 0, 0, 0], port)).await;
}
