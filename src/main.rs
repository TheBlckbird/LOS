#![no_main]
#![no_std]

mod vga_buffer;

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // use core::fmt::Write;
    // vga_buffer::WRITER.lock().write_str("Hello again").unwrap();
    // write!(
    //     vga_buffer::WRITER.lock(),
    //     ", some numbers: {} {}",
    //     42,
    //     1.337
    // )
    // .unwrap();

    println!("Hello World{}", "!");
    println!("Hello World{}", "!");
    println!("some numbers: {} {}", 42, 1.337);

    #[allow(clippy::empty_loop)]
    loop {}
}

/// This function is called on panic.

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}
