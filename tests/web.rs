//! Test suite for the Web and headless browsers.
#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;
use neomatrixcode_hello_wasm::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn pass() {
    assert_eq!(add_two_ints(10, 20), 30);
    assert_eq!(fib(10), 55);
    assert_eq!(net("192.168.0.20:9100"), 1);
}
