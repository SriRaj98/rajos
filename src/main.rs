// Disable the standard library
#![no_std]
#![no_main] // Disable the standard main entry point

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// Define the entry point for the OS kernel
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    loop {}
}