#![allow(unused)]

use std::any::Any;

pub struct SlotMap {
    slots: Vec<(u32, Box<dyn Any>)>,
}

pub struct Entry {
    index: u32,
    generation: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_and_set_values() {}
}
