use spin_sdk::http::{IntoResponse, Request, Response};
use spin_sdk::http_component;

/// A simple Spin HTTP component.
#[http_component]
fn handle_hello_world_rust_spin_app(req: Request) -> anyhow::Result<impl IntoResponse> {
    let url = req
        .header("spin-full-url")
        .map_or("", |v| v.as_str().unwrap_or_default());
    println!("Handling request to {:?}", url);
    Ok(Response::builder()
        .status(200)
        .header("content-type", "text/plain")
        .body("Hello, Fermyon")
        .build())
}
