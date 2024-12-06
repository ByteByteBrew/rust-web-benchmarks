use astra::{Body, ConnectionInfo, Request, Response, Server};
use common::{
    format_fortunes_html, get_fortunes, get_json_response, HELLO_WORLD, HTML_CONTENT_TYPE,
};
use http::header;

fn serve(req: Request, _conn: ConnectionInfo) -> Response {
    let (req, _) = req.into_parts();

    match req.uri.path() {
        "/plaintext" => {
            let mut res = Response::new(Body::new(HELLO_WORLD));
            res.headers_mut().insert(header::CONTENT_TYPE, "text/plain".parse().unwrap());
            res
        }
        "/json" => {
            let json = get_json_response();
            let mut res = Response::new(Body::new(serde_json::to_vec(&json).unwrap()));
            res.headers_mut()
                .insert(header::CONTENT_TYPE, "application/json".parse().unwrap());
            res
        }
        "/fortunes" => {
            let mut res = Response::new(Body::new(format_fortunes_html(get_fortunes())));
            res.headers_mut()
                .insert(header::CONTENT_TYPE, HTML_CONTENT_TYPE.parse().unwrap());
            res
        }
        _ => {
            let mut res = Response::new(Body::empty());
            *res.status_mut() = http::StatusCode::NOT_FOUND;
            res
        }
    }
}

fn main() {
    Server::bind("0.0.0.0:8080")
        .max_workers(num_cpus::get() * 20)
        .http1_pipeline_flush(true)
        .http1_only()
        .serve(serve)
        .expect("failed to start server");
}
