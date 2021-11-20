#![no_std]
#![no_main]
#![allow(warnings)]
#![feature(asm, llvm_asm)]

use core::panic::PanicInfo;

pub mod kb;
pub mod screen;
pub mod system;
use screen::Screen;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let mut screen = Screen::new(0, 0, 80, 25);
    screen.print("\nEnter your name: ");
    keyboard_input!(screen, name);
    screen.print("\nHello, ");
    screen.print(name);
    screen.print("!");
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    loop {}
}
