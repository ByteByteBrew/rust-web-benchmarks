use common::{format_fortunes_html, get_fortunes, get_json_response, HELLO_WORLD};
use salvo::prelude::*;

#[handler]
async fn plaintext(res: &mut Response) {
    res.render(Text::Plain(HELLO_WORLD));
}

#[handler]
async fn json(res: &mut Response) {
    res.render(Json(get_json_response()));
}

#[handler]
async fn fortunes(res: &mut Response) {
    res.render(Text::Html(format_fortunes_html(get_fortunes())));
}

#[tokio::main]
async fn main() {
    let router = Router::new()
        .push(Router::with_path("plaintext").get(plaintext))
        .push(Router::with_path("json").get(json))
        .push(Router::with_path("fortunes").get(fortunes));

    println!("Server running on http://127.0.0.1:8080");
    let acceptor = TcpListener::new("127.0.0.1:8080").bind().await;
    Server::new(acceptor).serve(router).await;
}
