#![no_std]
#![no_main]
#![allow(warnings)]

use core::panic::PanicInfo;

pub mod screen;
use screen::Screen;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let mut screen = Screen::new(0, 0, 80, 25);
    screen.print("Hello World!");
    screen.print("\nThis is an emoji 🎈");
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    loop {}
}
