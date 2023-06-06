#![no_std] //disable link to standard library
#![no_main]
#[no_mangle] // disable name mangling

pub extern "C" fn _start() -> ! { //tell compiler that it should use the C calling convention for this function
    loop {}
}

use core::panic::PanicInfo;

// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}