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
    let mut keyboard_input = kb::KeyboardInput::new(&mut screen);
    let name = keyboard_input.read_str();
    screen.print("\nHello, ");
    screen.print(core::str::from_utf8(&name).unwrap());
    screen.print("!");
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    loop {}
}
