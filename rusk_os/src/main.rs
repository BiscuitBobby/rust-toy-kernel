/*cargo build --target x86_64-rusk.json
  cargo bootimage
  qemu-system-x86_64 -drive format=raw,file=target/x86_64-rusk/debug/bootimage-crate_name.bin
*/
#![no_std] //disable link to standard library
#![no_main]

use core::panic::PanicInfo;

// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! 
{
    loop {}
}

static HELLO: &[u8] = b"Boot succesful";

#[no_mangle] // disable name mangling
pub extern "C" fn _start() -> ! { //tell compiler that it should use the C calling convention for this function
    let vga_buffer = 0xb8000 as *mut u8; //buffer is located at address 0xb8000
    let keyboard_status_port: u16 = 0x64;
    let keyboard_data_port: u16 = 0x60;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }
    loop {}
}
