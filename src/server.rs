use crate::{
    api::{assets::assets, hello::hello, webauthn::webauthn},
    handle_rejection,
    services::webauthn::Webauthn,
    {config::Config, handlers, routes},
};

use std::{convert::Infallible, net::SocketAddr, sync::Arc};
use tokio::sync::Semaphore;
use tower::{limit::GlobalConcurrencyLimitLayer, ServiceBuilder};
use warp::{
    hyper::{server::conn::AddrStream, service::make_service_fn, Body, Response, Server},
    Filter,
};

const MAX_INFLIGHT_REQUESTS: usize = 5;
const MAX_CONNS: usize = 50;

#[derive(Clone)]
pub struct ServerProps {
    pub config: Arc<Config>,
    pub webauthn: Arc<Webauthn>,
}

pub async fn serve(config: Arc<Config>, webauthn: Arc<Webauthn>) {
    let conns_limit = Arc::new(Semaphore::new(MAX_CONNS));
    let reqs_limit = GlobalConcurrencyLimitLayer::new(MAX_INFLIGHT_REQUESTS);

    let app = make_service_fn(move |_stream: &AddrStream| {
        let conns_limit = conns_limit.clone();
        let reqs_limit = reqs_limit.clone();
        let webauthn = webauthn.clone();
        
        async move {
            let permit = Arc::new(conns_limit.acquire_owned().await.unwrap());
            let options = warp::options().map(|| warp::reply()).map(|reply| {
                warp::reply::with_header(reply, "Access-Control-Allow-Headers", "Content-Type")
            });
            let end = options
                .or(assets!()
                    .or(hello!())
                    .or(webauthn!(webauthn.clone()))
                    .map(|reply| {
                        warp::reply::with_header(reply, "Access-Control-Allow-Origin", "*")
                    }))
                .recover(handle_rejection)
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

    let socket_address = config
        .clone()
        .app_addr
        .parse::<SocketAddr>()
        .expect("Could not parse Addr");

    log::info!("👂Listening at {}", config.app_addr);
    Server::bind(&socket_address).serve(app).await.unwrap()
}
