use argan::{data::json::Json, prelude::*};
use common::{
    format_fortunes_html, get_fortunes, get_json_response, HELLO_WORLD, HTML_CONTENT_TYPE,
};
use hyper_util::{rt::TokioExecutor, server::conn::auto::Builder};

async fn plaintext() -> &'static str {
    HELLO_WORLD
}

async fn json() -> Json<&'static common::JsonResponse<'static>> {
    Json(get_json_response())
}

async fn fortunes() -> Response {
    Response::builder()
        .header("content-type", HTML_CONTENT_TYPE)
        .body(format_fortunes_html(get_fortunes()).into())
        .unwrap()
}

#[tokio::main]
async fn main() -> Result<(), BoxedError> {
    let mut router = Router::new();

    router.resource_mut("/plaintext").set_handler_for(Method::GET.to(plaintext));

    router.resource_mut("/json").set_handler_for(Method::GET.to(json));

    router.resource_mut("/fortunes").set_handler_for(Method::GET.to(fortunes));

    let arc_service = router.into_arc_service();
    let connection_builder = Builder::new(TokioExecutor::new());

    println!("Server running on http://127.0.0.1:8080");
    Server::new(connection_builder)
        .serve(arc_service, "127.0.0.1:8080")
        .await
        .map_err(Into::into)
}
