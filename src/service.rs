use std::sync::{Arc, Mutex};

use futures::SinkExt;
use http_body_util::Full;
use hyper::{
    body::{Bytes, Incoming},
    Method, Request, Response, StatusCode,
};
use hyper_tungstenite::HyperWebsocket;
use tokio::sync::broadcast::Sender;
use tungstenite::Message;

use crate::{config::Config, signal::Signal, Result};

type Body = Full<Bytes>;
type SignalSender = Arc<Sender<Signal>>;

static INDEX: &[u8] = include_bytes!("../index.html");
static FONT: &[u8] = include_bytes!("../assets/fonts/Rubik.woff2");

fn full<T: Into<Bytes>>(chunk: T) -> Body {
    Full::new(chunk.into())
}

async fn websocket_handler(websocket: HyperWebsocket, signal: SignalSender) -> Result<()> {
    let mut websocket = websocket.await?;
    let mut rx = signal.subscribe();

    while let Ok(value) = rx.recv().await {
        websocket
            .send(Message::Text(value.unicode_character()))
            .await?;
    }

    Ok(())
}

pub async fn request_handler(
    mut req: Request<Incoming>,
    config: Arc<Mutex<Config>>,
    signal: SignalSender,
) -> Result<Response<Body>> {
    let payload = (
        req.method(),
        req.uri().path(),
        hyper_tungstenite::is_upgrade_request(&req),
    );

    match payload {
        (&Method::GET, "/", false) => {
            let config = config.lock().unwrap();

            let res = Response::new(full(Bytes::from_static(INDEX)));
            let (mut parts, body) = res.into_parts();

            parts.headers = config.to_cookie_headers();
            let res = Response::from_parts(parts, body);

            Ok(res)
        }
        (&Method::GET, "/Rubik.woff2", false) => {
            let res = Response::new(full(Bytes::from_static(FONT)));

            Ok(res)
        }
        (&Method::GET, "/signal", true) => {
            let (res, websocket) = hyper_tungstenite::upgrade(&mut req, None)?;

            tokio::spawn(async move {
                if let Err(err) = websocket_handler(websocket, signal).await {
                    eprintln!("WebSocket: {:?}", err);
                }
            });

            Ok(res)
        }
        _ => {
            // 404
            Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(full(""))
                .unwrap())
        }
    }
}
