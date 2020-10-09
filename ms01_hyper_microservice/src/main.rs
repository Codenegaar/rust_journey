use std::convert::Infallible;
use std::net::SocketAddr;

use hyper::{Body, Request, Response, Server,
            Method, StatusCode
            };
use hyper::service::{make_service_fn, service_fn};

use futures::TryStreamExt as _;

//Logging and conf.
use log::{debug, info, trace, error};
use clap::{App, crate_name, crate_authors, crate_description, crate_version, Arg};

#[tokio::main]
async fn main() {
    //Start logging and configuration
    //Initialize logging
    pretty_env_logger::init();

    //Create a command-line parser
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(Arg::with_name("address")
            .short('a')
            .long("address")
            .value_name("ADDRESS")
            .help("IP address to bind the server to")
            .takes_value(true))
        .arg(Arg::with_name("port")
            .short("p")
            .long("port")
            .value_name("PORT")
            .help("Port to listen to")
            .takes_value(true))
        .arg(Arg::with_name("log-level")
            .short("l")
            .long("log-level")
            .value_name("LOG_LEVEL")
            .help("Log level, trace, debug, info, warn, error")
            .takes_value(true))
        .get_matches();

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    //Create a service
    let make_svc = make_service_fn(|_conn| async {
        Ok::<_, Infallible>(service_fn(hello_world))
    });

    let server = Server::bind(&addr).serve(make_svc);
    let graceful = server.with_graceful_shutdown(shutdown_signal());

    info!("Starting the server");
    if let Err(e) = graceful.await {
        // eprintln!("Error registering grateful shutdown signal: {}", e);
        error!("Error registering graceful shutdown or server error: {}", e);
    }
}

async fn hello_world(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let mut response = Response::new(Body::empty());

    //Define routing table
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => {
            *response.body_mut() = Body::from("Hello bastard\n");
        },
        (&Method::POST, "/echo") => {
            *response.body_mut() = req.into_body();
        },
        (&Method::POST, "/echo/uppercase") => {
            let mapping = req
                .into_body()
                .map_ok(|chunk| {
                    chunk.iter()
                        .map(|byte| byte.to_ascii_uppercase())
                        .collect::<Vec<u8>>()
                });
            *response.body_mut() = Body::wrap_stream(mapping);
        },
        (&Method::POST, "/echo/reversed") => {
            let full_body = hyper::body::to_bytes(req.into_body()).await;
            if let Ok(full_body) = full_body {
                let reversed = full_body.iter()
                    .rev()
                    .cloned()
                    .collect::<Vec<u8>>();

                *response.body_mut() = reversed.into();
            } else {
                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
            }
        },
        _ => {
            *response.status_mut() = StatusCode::NOT_FOUND;
        },
    };

    Ok(response)
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to register Ctrl + C signal handler");
}