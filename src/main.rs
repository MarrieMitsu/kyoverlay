use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::thread;

use clap::Parser;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper_util::rt::TokioIo;
use inputbot::KeybdKey::{self, *};
use kyoverlay::config::Config;
use kyoverlay::service::request_handler;
use kyoverlay::signal::Signal;
use kyoverlay::Result;
use tokio::net::TcpListener;
use tokio::sync::broadcast;

#[tokio::main]
async fn main() -> Result<()> {
    let config = Arc::new(Mutex::new(Config::parse()));

    let (tx, _) = broadcast::channel::<Signal>(32);
    let tx1 = Arc::new(tx.clone());

    KeybdKey::bind_all(move |event| {
        let _ = match event {
            LShiftKey | RShiftKey | LControlKey | RControlKey | LAltKey | RAltKey => {}
            _ => {
                let sig = Signal {
                    key: event,
                    is_capslock: CapsLockKey.is_toggled(),
                    is_shift: LShiftKey.is_pressed() || RShiftKey.is_pressed(),
                    is_ctrl: LControlKey.is_pressed() || RControlKey.is_pressed(),
                    is_alt: LAltKey.is_pressed() || RAltKey.is_pressed(),
                };

                if let Err(_) = tx.send(sig) {}
            }
        };
    });

    thread::spawn(inputbot::handle_input_events);

    let addr = SocketAddr::from(([127, 0, 0, 1], config.lock().unwrap().port));
    let listener = TcpListener::bind(addr).await?;

    println!("Listening on http://{}", addr);

    let mut http = http1::Builder::new();
    http.keep_alive(true);

    loop {
        let (stream, _) = listener.accept().await?;

        let io = TokioIo::new(stream);
        let tx = tx1.clone();
        let config = config.clone();
        let service = service_fn(move |req| request_handler(req, config.clone(), tx.clone()));

        let connection = http.serve_connection(io, service).with_upgrades();

        tokio::spawn(async move {
            if let Err(err) = connection.await {
                eprintln!("Server: {:?}", err);
            }
        });
    }
}
