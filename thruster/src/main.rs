use common::{
    format_fortunes_html, get_fortunes, get_json_response, HELLO_WORLD, HTML_CONTENT_TYPE,
};
use serde_json;
use thruster::{
    m, middleware_fn, App, BasicContext as Ctx, Context, MiddlewareNext, MiddlewareResult, Request,
    Server, ThrusterServer,
};

#[middleware_fn]
async fn plaintext(mut ctx: Ctx, _next: MiddlewareNext<Ctx>) -> MiddlewareResult<Ctx> {
    ctx.body(HELLO_WORLD);
    ctx.set("Content-Type", "text/plain");
    Ok(ctx)
}

#[middleware_fn]
async fn json(mut ctx: Ctx, _next: MiddlewareNext<Ctx>) -> MiddlewareResult<Ctx> {
    ctx.body(&serde_json::to_string(&get_json_response()).unwrap());
    ctx.set("Content-Type", "application/json");
    Ok(ctx)
}

#[middleware_fn]
async fn fortunes(mut ctx: Ctx, _next: MiddlewareNext<Ctx>) -> MiddlewareResult<Ctx> {
    ctx.body(&format_fortunes_html(get_fortunes()));
    ctx.set("Content-Type", HTML_CONTENT_TYPE);
    Ok(ctx)
}

#[tokio::main]
async fn main() {
    println!("Server running on http://0.0.0.0:8080");

    let app = App::<Request, Ctx, ()>::new_basic()
        .get("/plaintext", m![plaintext])
        .get("/json", m![json])
        .get("/fortunes", m![fortunes]);

    let server = Server::new(app);
    server.build("0.0.0.0", 8080).await;
}
