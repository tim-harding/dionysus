mod console;

use bevy_ecs::{
    prelude::{Component, Entity},
    schedule::Schedule,
    system::Query,
    world::World,
};
use std::panic;
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{window, HtmlButtonElement, HtmlInputElement, HtmlUListElement};

// NOTE:
// Programming as if weak refs are available everywhere because it makes
// programming easier. Don't have to worry about managing the lifetime of
// callbacks for DOM elements.

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
    let mut schedule = Schedule::default();
    schedule.add_systems(display_todos);

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
    let update_ul = Closure::<dyn FnMut()>::new(move || {});
    let closure = Closure::<dyn FnMut()>::new(move || {
        world.spawn(Todo::new(input.value()));
        input.set_value("");
        schedule.run(&mut world);
    });
    button
        .add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
        .unwrap();
    closure.forget();
    body.append_child(&button).unwrap();
}
