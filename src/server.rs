use std::{
    convert::Infallible,
    num::NonZeroUsize,
    task::{Context, Poll},
};

use anyhow::Result;
use axum::{
    body::Body,
    extract::State,
    http::{
        header::{CACHE_CONTROL, CONTENT_TYPE},
        HeaderValue, Request,
    },
    response::{
        sse::{Event, Sse},
        IntoResponse, Response,
    },
};
use futures_util::{future::BoxFuture, Stream};
use tokio::sync::watch::Receiver;
use tower::{Layer, Service};

use crate::{config::ServerConfig, site::Site};

pub async fn serve(config: ServerConfig) -> Result<()> {
    let cache = jobber::Cache::new(NonZeroUsize::new(1024).unwrap());
    let watch_dir = config.build_config.root_dir.clone();
    let serve_dir = config.build_config.output_dir.clone();
    let site = Site::new(config.build_config, config.hot_reload);

    // Initial build
    let h = match site.build_site(&cache) {
        Ok(((), h)) => h,
        Err(err) => {
            println!("Error building: {err:#}");
            0
        }
    };

    // Watch for file changes
    let (tx, rx) = tokio::sync::watch::channel(h);
    let mut debouncer = notify_debouncer_mini::new_debouncer(
        std::time::Duration::from_millis(500),
        move |_res: notify_debouncer_mini::DebounceEventResult| {
            tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap()
                .block_on(async {
                    match site.build_site(&cache) {
                        Ok(((), h)) => {
                            tx.send(h).unwrap();
                        }
                        Err(err) => println!("Error rebuilding: {err:#}"),
                    }
                });
        },
    )?;
    debouncer.watcher().watch(
        &watch_dir,
        notify_debouncer_mini::notify::RecursiveMode::Recursive,
    )?;

    let dir_service = tower_http::services::ServeDir::new(&serve_dir);
    println!("Listening on {:?}", &config.addr);
    let mut service = axum::Router::new();
    if !config.http_cache {
        service = service.layer(NoCacheLayer);
    }
    service = service.nest_service("/", dir_service);
    if config.hot_reload {
        service = service
            .route("/hr.js", axum::routing::get(sse_script_handler))
            .route("/hr", axum::routing::get(sse_handler));
    }
    let app = service.with_state(rx);
    axum::Server::bind(&config.addr)
        .serve(app.into_make_service())
        .await?;
    Ok(())
}

async fn sse_script_handler(State(rx): State<Receiver<u64>>) -> impl IntoResponse {
    let contents = include_str!("./hr.js").replace("%HOTRELOADID%", &format!("{}", *rx.borrow()));
    ([(CONTENT_TYPE, "text/javascript")], contents)
}

async fn sse_handler(
    State(rx): State<Receiver<u64>>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    use tokio_stream::StreamExt;
    let stream = tokio_stream::wrappers::WatchStream::new(rx)
        .map(|i| Ok(Event::default().data(format!("{i}"))));
    Sse::new(stream)
}

#[derive(Debug, Clone)]
struct NoCacheLayer;

impl<S> Layer<S> for NoCacheLayer {
    type Service = NoCacheMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        NoCacheMiddleware { inner }
    }
}

#[derive(Debug, Clone)]
struct NoCacheMiddleware<S> {
    inner: S,
}

impl<S> Service<Request<Body>> for NoCacheMiddleware<S>
where
    S: Service<Request<Body>, Response = Response> + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, request: Request<Body>) -> Self::Future {
        let future = self.inner.call(request);
        Box::pin(async move {
            let mut response: Response = future.await?;
            let headers = response.headers_mut();
            headers.insert(CACHE_CONTROL, HeaderValue::from_static("no-cache"));
            Ok(response)
        })
    }
}
