use crate::{
    api::{assets::assets, page::page, user::user},
    config::{Config},
    db_conn::DbConn,
    handle_final_rejection, handle_rejection, handlers, routes,
};

use tower_http::{
    add_extension::AddExtensionLayer,
    compression::CompressionLayer,
    sensitive_headers::SetSensitiveHeadersLayer,
    trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer},
    LatencyUnit,
};

use hyper::{
    header,
    server::conn::AddrStream,
    service::make_service_fn,
    Body, Response, Server,
};
use bytes::Bytes;
use std::{
    convert::Infallible,
    net::{TcpListener},
    sync::Arc,
    time::Duration,
};
use tokio::{sync::Semaphore, time::timeout};
use tower::{limit::GlobalConcurrencyLimitLayer, ServiceBuilder};
use warp::Filter;

const CONN_TIMEOUT: u64 = 2 * 60;
const REQ_TIMEOUT: u64 = 2 * 60;

pub async fn serve(listener: TcpListener, config: Arc<Config>) -> Result<(), warp::hyper::Error> {
    let conns_limit = Arc::new(Semaphore::new(config.clone().max_conn));
    let reqs_limit = GlobalConcurrencyLimitLayer::new(config.clone().max_reqs);

    let db_conn = Arc::new(DbConn::new(&config.db_path));
    let context = Context::new(config.clone(), db_conn.clone());

    let options = warp::options().map(|| warp::reply()).map(|reply| {
        warp::reply::with_header(reply, "Access-Control-Allow-Headers", "Content-Type")
    });

    let end = options
        .or(user!()
            // .or(hello!())
            .or(page!())
            .or(assets!())
            .map(|reply| warp::reply::with_header(reply, "Access-Control-Allow-Origin", "*")))
        .recover(handle_final_rejection);

    let app = make_service_fn(move |_stream: &AddrStream| {
        let conns_limit = conns_limit.clone();
        let reqs_limit = reqs_limit.clone();

        let context = context.clone();
        let end = end.clone();

        async move {
            let future_permit = conns_limit.acquire_owned();
            let conn_timeout = Duration::from_secs(CONN_TIMEOUT);
            let expected_permit = timeout(conn_timeout, future_permit).await.unwrap().unwrap();
            let permit = Arc::new(expected_permit);

            Ok::<_, Infallible>(
                ServiceBuilder::new()
                    .layer(reqs_limit)
                    .layer(
                        TraceLayer::new_for_http()
                            .on_body_chunk(|chunk: &Bytes, latency: Duration, _: &tracing::Span| {
                                tracing::trace!(size_bytes = chunk.len(), latency = ?latency, "sending body chunk")
                            })
                            .make_span_with(DefaultMakeSpan::new().include_headers(true))
                            .on_response(DefaultOnResponse::new().include_headers(true).latency_unit(LatencyUnit::Micros)),
                    )
                    // Set a timeout
                    .timeout(Duration::from_secs(REQ_TIMEOUT))
                    // Share the context with each handler via a request extension
                    .layer(AddExtensionLayer::new(context))
                    // Compress responses
                    .layer(CompressionLayer::new())
                    // Mark the `Authorization` and `Cookie` headers as sensitive so it doesn't show in logs
                    .layer(SetSensitiveHeadersLayer::new(vec![
                        header::AUTHORIZATION,
                        header::COOKIE,
                    ]))
                    .then(move |res: Result<Response<Body>, Infallible>| {
                        drop(permit);
                        std::future::ready(res)
                    })
                    .service(warp::service(end)),
            )
        }
    });

    let addr = listener.local_addr().unwrap();
    tracing::info!("👂 Listening on {}", addr);

    Server::from_tcp(listener).unwrap().serve(app).await?;

    Ok(())
}

#[derive(Clone)]
pub struct Context {
    pub config: Arc<Config>,
    pub db_conn: Arc<DbConn>,
}

impl Context {
    pub fn new(config: Arc<Config>, db_conn: Arc<DbConn>) -> Self {
        Context {
            config: config,
            db_conn,
        }
    }
}
