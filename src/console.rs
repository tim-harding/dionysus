#![allow(unused_imports)]

#[macro_export]
macro_rules! log {
    ($($t:tt)*) => (web_sys::console::log_1(&format_args!($($t)*).to_string().into()))
}
pub use log;

#[macro_export]
macro_rules! trace {
    ($($t:tt)*) => (web_sys::console::trace_1(&format_args!($($t)*).to_string().into()))
}
pub use trace;

#[macro_export]
macro_rules! debug {
    ($($t:tt)*) => (web_sys::console::debug_1(&format_args!($($t)*).to_string().into()))
}
pub use debug;

#[macro_export]
macro_rules! info {
    ($($t:tt)*) => (web_sys::console::info_1(&format_args!($($t)*).to_string().into()))
}
pub use info;

#[macro_export]
macro_rules! warn_inner {
    ($($t:tt)*) => (web_sys::console::warn_1(&format_args!($($t)*).to_string().into()))
}
pub use warn_inner as warn;

#[macro_export]
macro_rules! error {
    ($($t:tt)*) => (web_sys::console::error_1(&format_args!($($t)*).to_string().into()))
}
pub use error;
