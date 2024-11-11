use std::io;

use bytes::BufMut;
use common::{format_fortunes_html, get_fortunes, get_json_response, HELLO_WORLD};
use may_minihttp::{HttpServer, HttpService, Request, Response};

#[derive(Clone)]
struct WebService;

impl HttpService for WebService {
    fn call(&mut self, req: Request, rsp: &mut Response) -> io::Result<()> {
        match req.path() {
            "/plaintext" => {
                rsp.body_mut().put_slice(HELLO_WORLD.as_bytes());
                Ok(())
            }
            "/json" => {
                rsp.header("Content-Type: application/json");
                let w = rsp.body_mut().writer();
                serde_json::to_writer(w, get_json_response())?;
                Ok(())
            }
            "/fortunes" => {
                rsp.header("Content-Type: text/html; charset=utf-8");
                let html = format_fortunes_html(get_fortunes());
                rsp.body_mut().put_slice(html.as_bytes());
                Ok(())
            }
            _ => {
                rsp.status_code(404, "Not Found");
                Ok(())
            }
        }
    }
}

fn main() {
    println!("Server running on http://127.0.0.1:8080");
    let server = HttpServer(WebService).start("127.0.0.1:8080").unwrap();
    server.wait();
}
