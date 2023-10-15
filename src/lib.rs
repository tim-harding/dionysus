pub mod console;
pub mod signal;
pub mod slot_map;

#[cfg(test)]
pub mod tests {
    use wasm_bindgen_test::*;
    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn it_works() {
        assert_eq!(1, 1);
    }
}
