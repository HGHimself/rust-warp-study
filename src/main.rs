use env_logger::Env;
use rust_warp_study::{handlers::hello_handler, routes::hello_route};
use std::{convert::Infallible, sync::Arc};
use tokio::sync::Semaphore;
use tower::{limit::GlobalConcurrencyLimitLayer, ServiceBuilder};
use warp::{
    hyper::{server::conn::AddrStream, service::make_service_fn, Body, Response, Server},
    Filter,
};

pub mod api;

const MAX_INFLIGHT_REQUESTS: usize = 5;
const MAX_CONNS: usize = 50;

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let conns_limit = Arc::new(Semaphore::new(MAX_CONNS));
    let reqs_limit = GlobalConcurrencyLimitLayer::new(MAX_INFLIGHT_REQUESTS);

    let app = make_service_fn(move |_stream: &AddrStream| {
        let conns_limit = conns_limit.clone();
        let reqs_limit = reqs_limit.clone();
        
        async move {
            let permit = Arc::new(conns_limit.acquire_owned().await.unwrap());
            let end = hello!()
                .recover(rust_warp_study::handle_rejection)
                .with(warp::log("hello warp"));

            Ok::<_, Infallible>(
                ServiceBuilder::new()
                    .layer(reqs_limit)
                    .then(move |res: Result<Response<Body>, Infallible>| {
                        drop(permit);
                        std::future::ready(res)
                    })
                    .service(warp::service(end)),
            )
        }
    });

    Server::bind(&([127, 0, 0, 1], 1025).into())
        .serve(app)
        .await
        .unwrap();
}
