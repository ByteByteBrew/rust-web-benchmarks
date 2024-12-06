use common::{format_fortunes_html, get_fortunes, get_json_response, HELLO_WORLD};
use rouille::{router, Response};

fn main() {
    rouille::start_server("0.0.0.0:8080", move |request| {
        router!(request,
            (GET) (/plaintext) => {
                Response::text(HELLO_WORLD)
            },
            (GET) (/json) => {
                Response::json(&get_json_response())
            },
            (GET) (/fortunes) => {
                Response::html(format_fortunes_html(get_fortunes()))
            },
            _ => Response::empty_404()
        )
    });
}
