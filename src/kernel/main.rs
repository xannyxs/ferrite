#![no_std]
#![no_main]

#[cfg(not(any(target_arch = "x86")))]
compile_error!("This code needs to be compiled for i686/x86!");

#[cfg(not(target_pointer_width = "32"))]
compile_error!("This code needs to be compiled for 32-bit architecture!");

use core::panic::PanicInfo;

const VGA_BUFFER: *mut u8 = 0xB8000 as *mut u8;
const VGA_WIDTH: usize = 80;   // Standard VGA text mode is 80x25
const VGA_HEIGHT: usize = 25;
const CLEAR_COLOR: u8 = 0x0F;  // Black background (0) with White foreground (F)
const CLEAR_CHAR: u8 = b' ';   // Space character for clearing

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    unsafe {
        for i in 0..(VGA_WIDTH * VGA_HEIGHT) {
            let offset = i * 2;
            
            *VGA_BUFFER.offset(offset as isize) = CLEAR_CHAR;
            *VGA_BUFFER.offset((offset + 1) as isize) = CLEAR_COLOR;
        }

        let hello = b"Hello World!";
        for (i, &byte) in hello.iter().enumerate() {
            *VGA_BUFFER.offset(i as isize * 2) = byte;
            *VGA_BUFFER.offset(i as isize * 2 + 1) = CLEAR_COLOR;
        }
    }

    loop {
        unsafe {
            core::arch::asm!("hlt", options(nomem, nostack));
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
