use common::{format_fortunes_html, get_fortunes, get_json_response, HELLO_WORLD};
use oxidy::{route, Context, Returns, Server};

async fn plaintext(mut c: Context) -> Returns {
    c.response.body = HELLO_WORLD.to_string();
    c.response.content_type = "text/plain".into();
    (c, None)
}

async fn json(mut c: Context) -> Returns {
    c.response.body = serde_json::to_string(&get_json_response()).unwrap();
    c.response.content_type = "application/json".into();
    (c, None)
}

async fn fortunes(mut c: Context) -> Returns {
    c.response.body = format_fortunes_html(get_fortunes());
    c.response.content_type = "text/html; charset=utf-8".into();
    (c, None)
}

#[tokio::main]
async fn main() {
    let mut app = Server::new();
    app.add(route!("get /plaintext", plaintext));
    app.add(route!("get /json", json));
    app.add(route!("get /fortunes", fortunes));
    app.run("127.0.0.1:8080").await;
}
