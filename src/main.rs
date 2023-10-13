use std::panic;
use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

fn main() {
    println!("Hello, world!");
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    panic!("Panicking aaaaa");
}
