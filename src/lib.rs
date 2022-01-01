mod utils;
use std::net::TcpStream;
use std::io::Write;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, neomatrixcode_hello_wasm!");
}

#[wasm_bindgen]
pub fn add_two_ints(a: u32, b: u32) -> u32 {
   a + b
}

#[wasm_bindgen]
pub fn fib(n: u32) -> u32 {
   if n == 0 || n == 1 {
      return n;
   }

   fib(n - 1) + fib(n - 2)
}

#[wasm_bindgen]
pub fn net(ip: &str) -> u32 {
    let mut stream = TcpStream::connect(ip);
    stream.unwrap().write(&[0x1b, 0x74, 18, 0x48, 0x6f,0x6c ,0x61, 0x20 ,0x6d, 0x75, 0x6e, 0x64 ,0x6f ,0x21,0x1b, 0x64, 0x04 ,0x1b, 0x64, 0x04, 0x1d, 0x56, 0x00, 0x1b, 0x40]);
     Ok::<(), std::io::Error>(());

    return 1;

}