use dionysus::console;
use std::panic;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

fn main() {
    console::log!("Hello");
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    panic!("Panicking aaaaa");
}
