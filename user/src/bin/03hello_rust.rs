#![no_std]
#![no_main]

use core::arch::asm;

#[macro_use]
extern crate user_lib;

#[no_mangle]
fn main() -> i32 {
    let ma = 1;
    let mb = 2;
    let n:i32;

    n = ma + mb;
    println!("I have {} apple",ma);
    println!("You have {} apples",mb);
    println!("So we have {} apples",n);
            unsafe {
                        asm!("sret");
                            }
                0
}

