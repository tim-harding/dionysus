// NOTE:
// Instead of building up a tree, just construct a list of DOM fragments as text and a separate
// list of signals and nesting info to find the right nodes at runtime. Ideally do this at compile
// time.

// NOTE:
// Reactivity features to support
// - Text content
// - Slots
// - DOM refs / directives
// - Lifecycle
// - Bindings (including spread, arbitrary props)

// NOTE:
// Possible structure:
// Instead of components in the traditional sense, lean into fine-grained reactivity and make
// components in ECS just be DOM nodes. Queries are for e.g. (El, Slot), (El, TextContent), (El,
// Bindings), etc. to perform fine-grained updates. User code creates DOM fragments and updates
// Slot, TextContent, etc., then ECS reactivity system picks up changes and applies them to the
// DOM.

use bevy_ecs::prelude::*;
use bevy_utils::synccell::SyncCell;
use web_sys::HtmlElement;

/// A wrapper around [`HtmlElement`] that implements [`Send`] and [`Sync`]. This is okay in
/// practice because WASM is only ever single-threaded, although this is something to pay attention
/// to if threading comes to WASM. Also need to pay attention and clone HtmlElement in public APIs
/// instead of handing out references to the inner type, which would be problematic in the presence
/// of any future threading API.
#[derive(Component)]
struct Element(HtmlElement);
unsafe impl Send for Element {}
unsafe impl Sync for Element {}

#[derive(Debug, Default)]
pub struct Cult {
    world: World,
}

impl Cult {
    pub fn new() -> Self {
        Self::default()
    }
}

type S = &'static str;
type Attributes = &'static [Attribute];
type Children = &'static [Child];

pub struct Tag(S, Attributes, Children);

pub enum Attribute {
    Static(&'static str, &'static str),
    Property(&'static str),
    Event(&'static str),
    Binding(Entity),
}

pub enum Child {
    Tag(Tag),
    TextStatic(&'static str),
    Text,
    Fragment(fn() -> Tag),
}

pub struct Template {
    tag: &'static str,
    slots: &'static [&'static [Breadcrumb]],
}

pub enum Breadcrumb {
    Child,
    Sibling,
}

// NOTE:
// It seems that abstracting Tag with functions, even const ones, is ergonomically problematic.
// Still possible if calls are wrapped with const{}, but that's not ideal. Probably macros are the
// best chance for API ergonomics if sticking with this route.

// NOTE:
// As a follow-up, see if it's possible to flatten this structure into HTML fragments in const
// context.

const fn api_test() -> Tag {
    Tag(
        "div",
        &[
            Attribute::Static("class", "m-5 p-2"),
            Attribute::Static("id", "my-id"),
        ],
        &[
            Child::Tag(Tag(
                "a",
                &[
                    Attribute::Static("href", "https://www.apple.com/"),
                    Attribute::Property("id"),
                ],
                &[Child::TextStatic("Apple")],
            )),
            Child::Tag(Tag(
                "a",
                &[Attribute::Static("href", "https://www.timharding.co/")],
                &[Child::Text],
            )),
        ],
    )
}
