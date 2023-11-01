use futures::stream::Stream;
use hyper::rt::Future;
use hyper::service::service_fn;
use hyper::{Body, Request, Response, Server, StatusCode};

type BoxFuture = Box<dyn Future<Item = Response<Body>, Error = hyper::Error> + Send>;
type Error = Box<dyn std::error::Error>;

fn finalize(result: Result<Vec<u8>, Error>) -> Response<Body> {
    match result {
        Ok(bytes) => Response::new(Body::from(bytes)),
        Err(error) => {
            let body = format!(
                "{{\"status\": \"{}\", \"description\":\"{}\"}}",
                StatusCode::INTERNAL_SERVER_ERROR.to_string(),
                error.to_string()
            );
            let mut resp = Response::new(Body::from(body));
            *resp.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
            resp
        }
    }
}

fn handler_service(req: Request<Body>) -> BoxFuture {
    Box::new(req.into_body().concat2().map(move |chunk| {
        let body = chunk.iter().cloned().collect::<Vec<u8>>();
        finalize(handler::handle(body))
    }))
}

fn main() {
    let addr = ([0, 0, 0, 0], 3000).into();

    let server = Server::bind(&addr)
        .serve(|| service_fn(handler_service))
        .map_err(|e| eprintln!("server error: {}", e));

    hyper::rt::run(server);
}
