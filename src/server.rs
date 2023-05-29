use crate::{
    api::{assets::assets, hello::hello},
    handle_rejection, is_static,
    {config::Config, handlers, routes},
};

use bytes::Bytes;
use tower_http::{
    add_extension::AddExtensionLayer,
    compression::CompressionLayer,
    sensitive_headers::SetSensitiveHeadersLayer,
    set_header::SetResponseHeaderLayer,
    trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer},
    LatencyUnit,
};

use hyper::{
    body::HttpBody,
    header::{self, HeaderValue},
    server::conn::AddrStream,
    service::make_service_fn,
    Body, Response, Server, StatusCode,
};

use std::{
    convert::Infallible,
    net::{SocketAddr, TcpListener},
    sync::Arc,
    time::Duration,
};
use tokio::sync::Semaphore;
use tower::{limit::GlobalConcurrencyLimitLayer, make::Shared, ServiceBuilder};
use warp::Filter;

pub async fn serve(listener: TcpListener, config: Arc<Config>) -> Result<(), warp::hyper::Error> {
    let conns_limit = Arc::new(Semaphore::new(config.clone().max_conn));
    let reqs_limit = GlobalConcurrencyLimitLayer::new(config.clone().max_reqs);

    let state = State::new(config.clone());

    let app = make_service_fn(move |_stream: &AddrStream| {
        let conns_limit = conns_limit.clone();
        let reqs_limit = reqs_limit.clone();

        let state = state.clone();

        async move {
            log::info!("Grabbing permit");
            let permit = Arc::new(conns_limit.acquire_owned().await.unwrap());
            log::info!("Got the permit");

            let options = warp::options().map(|| warp::reply()).map(|reply| {
                warp::reply::with_header(reply, "Access-Control-Allow-Headers", "Content-Type")
            });

            let end = options
                .or(assets!().or(hello!()).map(|reply| {
                    warp::reply::with_header(reply, "Access-Control-Allow-Origin", "*")
                }))
                .recover(handle_rejection);

            let service = warp::service(end);

            Ok::<_, Infallible>(
                ServiceBuilder::new()
                    .layer(reqs_limit)
                    // Add high level tracing/logging to all requests
                    .layer(
                        TraceLayer::new_for_http()
                            .on_body_chunk(|chunk: &Bytes, latency: Duration, _: &tracing::Span| {
                                tracing::trace!(size_bytes = chunk.len(), latency = ?latency, "sending body chunk")
                            })
                            .make_span_with(DefaultMakeSpan::new().include_headers(true))
                            .on_response(DefaultOnResponse::new().include_headers(true).latency_unit(LatencyUnit::Micros)),
                    )
                    // Set a timeout
                    .timeout(Duration::from_secs(10))
                    // Share the state with each handler via a request extension
                    .layer(AddExtensionLayer::new(state))
                    // Compress responses
                    .layer(CompressionLayer::new())
                    // Mark the `Authorization` and `Cookie` headers as sensitive so it doesn't show in logs
                    .layer(SetSensitiveHeadersLayer::new(vec![
                        header::AUTHORIZATION,
                        header::COOKIE,
                    ]))
                    .then(move |res: Result<Response<Body>, Infallible>| {
                        drop(permit);
                        log::info!("releasing the permit");
                        std::future::ready(res)
                    })
                    .service(service),
            )
        }
    });

    let addr = listener.local_addr().unwrap();
    tracing::info!("👂 Listening on {}", addr);

    Server::from_tcp(listener).unwrap().serve(app).await?;

    Ok(())
}

#[derive(Clone)]
pub struct State {
    pub config: Arc<Config>,
}

impl State {
    pub fn new(config: Arc<Config>) -> Self {
        State { config: config }
    }
}
