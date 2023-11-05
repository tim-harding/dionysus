use bevy_ecs::{
    prelude::{Component, Entity},
    system::Query,
    world::World,
};
use dionysus::console;
use std::panic;
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{window, HtmlButtonElement, HtmlInputElement, HtmlUListElement};

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(Component)]
struct Completed;

#[derive(Component)]
struct Todo {
    pub text: String,
}

impl Todo {
    pub fn new(text: String) -> Self {
        Self { text }
    }
}

struct Reactive {
    effects: Vec<Box<dyn FnMut()>>,
}

fn display_todos(query: Query<(Entity, &Todo)>) {
    // for todo in query {}
}

fn main() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    let mut world = World::new();
    world.spawn(Todo::new("Hello".to_string()));
    world.spawn(Todo::new("world".to_string()));
    world.spawn((Todo::new("things".to_string()), Completed));
    world.spawn((Todo::new("stuff".to_string()), Completed));

    let window = window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();

    let ul: HtmlUListElement = document.create_element("ul").unwrap().dyn_into().unwrap();
    body.append_child(&ul).unwrap();

    let input: HtmlInputElement = document
        .create_element("input")
        .unwrap()
        .dyn_into()
        .unwrap();
    body.append_child(&input).unwrap();

    let button: HtmlButtonElement = document
        .create_element("button")
        .unwrap()
        .dyn_into()
        .unwrap();
    button.set_text_content(Some("Create"));
    let closure = Closure::<dyn FnMut()>::new(move || {
        world.spawn(Todo::new(input.value()));
        input.set_value("");
        console::log!("Hit");
    });
    button
        .add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
        .unwrap();
    closure.forget();
    body.append_child(&button).unwrap();
}
