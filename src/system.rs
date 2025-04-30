pub fn inportb(port: u16) -> u8 {
    let value: u8;
    unsafe {
        llvm_asm!("inb %dx, %al"
            : "={al}"(value)
            : "{dx}"(port)
            : "memory"
            : "volatile"
        );
    }
    value
}

pub fn outportb(port: u16, value: u8) {
    unsafe {
        llvm_asm!("outb %1, %0"
            :
            : "{dx}"(port), "{al}"(value)
            : "memory"
            : "intel", "volatile"
        );
    }
}
