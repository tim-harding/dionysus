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
