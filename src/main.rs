use std::panic;
use wasm_bindgen::JsCast;
use web_sys::{window, HtmlElement};

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

fn main() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    let window = window().unwrap();
    let document = window.document().unwrap();
    let p: HtmlElement = document.create_element("p").unwrap().dyn_into().unwrap();
    p.set_inner_text("Hello");
    let body = document.body().unwrap();
    body.append_child(&p).unwrap();
}
