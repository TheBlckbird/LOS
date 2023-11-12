#![no_main]
#![no_std]
#![feature(custom_test_frameworks)]
#![test_runner(test_runner)]
#![reexport_test_harness_main = "test_main"]

mod exit_qemu;
mod serial;
mod vga_buffer;

use core::panic::PanicInfo;
#[cfg(test)]
use exit_qemu::{exit_qemu, QemuExitCode};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");
    println!("Hello World{}", "!");
    println!("some numbers: {} {}", 42, 1.337);
    println!("Louis");

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

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    let tests_len = tests.len();
    serial_println!(
        "Running {} test{}",
        tests_len,
        if tests_len == 1 { "" } else { "s" }
    );

    for test in tests {
        test();
    }

    exit_qemu(QemuExitCode::Success);
}

// test suite helper
#[macro_export]
macro_rules! tests {
    {$($name:ident $body:block)*} => {
        $(
            #[cfg(test)]
            #[test_case]
            fn $name() {
                $crate::serial_print!("test ");
                for word in stringify!($name).split('_') {
                    $crate::serial_print!("{} ", word);
                }
                $crate::serial_print!("\t");
                $body
                $crate::serial_println!("[ok]");
            }
        )*
    };
}
