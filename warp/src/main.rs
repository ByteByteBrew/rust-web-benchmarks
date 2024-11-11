use common::{
    format_fortunes_html, get_fortunes, get_json_response, HELLO_WORLD, HTML_CONTENT_TYPE,
};
use warp::Filter;

#[tokio::main]
async fn main() {
    let plaintext = warp::path("plaintext").map(|| HELLO_WORLD);

    let json = warp::path("json").map(|| warp::reply::json(&get_json_response()));

    let fortunes = warp::path("fortunes").map(|| {
        warp::reply::with_header(
            format_fortunes_html(get_fortunes()),
            "content-type",
            HTML_CONTENT_TYPE,
        )
    });

    let routes = plaintext.or(json).or(fortunes);

    println!("Server running on http://127.0.0.1:8080");
    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
}
