use std::convert::Infallible;
use std::net::SocketAddr;
use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};
use hyper::{Method, StatusCode};
use futures::TryStreamExt as _;

async fn hello_world(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    println!("received request!");
    let mut response = Response::new(Body::empty());
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => {
            *response.body_mut() = Body::from("Try POSTing data to /echo");
        },
        (&Method::POST, "/echo") => {
            *response.body_mut() = req.into_body();
        },
        _ => {
            *response.status_mut() = StatusCode::NOT_FOUND;
        },
    };
    Ok(response)
}
#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let make_svc = make_service_fn(|_conn| async {
        Ok::<_, Infallible>(service_fn(hello_world))
    });
    let server = Server::bind(&addr).serve(make_svc);
    if let Err(e) = server.await {
        println!("server error: {}", e);
    }
}
