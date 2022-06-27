use env_logger::Env;
use rust_warp_study::{handlers::hello_handler, routes::hello_route};
use std::{
    convert::Infallible,
    future::{ready, Ready},
    sync::Arc,
    task::{Context, Poll},
    time::Duration,
};
use tokio::sync::Semaphore;
use tower::{limit::GlobalConcurrencyLimitLayer, ServiceBuilder};
use warp::{
    hyper::{
        server::conn::AddrStream,
        service::Service,
        service::{make_service_fn, service_fn},
        Body, Request, Response, Server,
    },
    Filter,
};

pub mod api;

const MAX_INFLIGHT_REQUESTS: usize = 5;

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let reqs_limit = GlobalConcurrencyLimitLayer::new(MAX_INFLIGHT_REQUESTS);
    let app = make_service_fn(move |_stream: &AddrStream| {
        let end = hello!()
            .recover(rust_warp_study::handle_rejection)
            .with(warp::log("hello warp"));

        std::future::ready(Ok::<_, Infallible>(
            ServiceBuilder::new()
                .layer(reqs_limit.clone())
                .service(warp::service(end)),
        ))
    });

    Server::bind(&([127, 0, 0, 1], 1025).into())
        .serve(app)
        .await
        .unwrap();
}
