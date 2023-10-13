use dionysus::console;
use std::panic;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::{future_to_promise, JsFuture};
use web_sys::{Request, RequestInit, RequestMode, Response};

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

fn main() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    let _ = future_to_promise(async move {
        match fetch_something().await {
            Ok(_) => console::log!("Resolved"),
            Err(_) => console::error!("Rejected"),
        }
        Ok(JsValue::null())
    });
}

async fn fetch_something() -> Result<JsValue, JsValue> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let request = Request::new_with_str_and_init("https://www.apple.com/", &opts)?;
    request
        .headers()
        .set("Accept", "application/vnd.github.v3+json")?;

    let window = web_sys::window().unwrap();
    console::log!("Request start");
    let response_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    console::log!("Request end");
    assert!(response_value.is_instance_of::<Response>());
    let response: Response = response_value.dyn_into().unwrap();
    console::log!("{}", response.status_text());
    Ok(JsValue::null())
}
