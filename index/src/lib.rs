use spin_sdk::http::{IntoResponse, Request, Response};
use spin_sdk::http_component;
/// A simple Spin HTTP component.
#[http_component]
fn handle_index(req: Request) -> anyhow::Result<impl IntoResponse> {
    println!("Handling request to {:?}", req.header("spin-full-url"));
    let body = include_str!("index.html");
    Ok(Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .body(body)
        .build())
}
