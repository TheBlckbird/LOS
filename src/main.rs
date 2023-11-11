#![no_main]
#![no_std]
#![feature(custom_test_frameworks)]
#![test_runner(testing::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod exit_qemu;
mod serial;
mod testing;
mod vga_buffer;

use core::panic::PanicInfo;
#[cfg(test)]
use exit_qemu::{exit_qemu, QemuExitCode};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");
    println!("Hello World{}", "!");
    println!("some numbers: {} {}", 42, 1.337);
    serial_println!("some numbers: {} {}", 42, 1.337);

    #[cfg(test)]
    test_main();

    #[allow(clippy::empty_loop)]
    loop {}
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

// our panic handler in test mode
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

tests! {
    testing_works {
        let one = 1;
        assert_eq!(1, one);
        assert_ne!(1, 4);
    }
}
