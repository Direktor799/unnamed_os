#![no_std]
#![no_main]

extern crate alloc;
extern crate user_lib;

use user_lib::*;

#[no_mangle]
fn main() -> i32 {
    println!("test");
    0
}
