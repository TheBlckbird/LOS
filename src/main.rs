#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(os_rust::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use os_rust::{println, tests};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    os_rust::init();

    println!("Hello World{}", "!");
    println!("Hello World{}", "!");
    println!("some numbers: {} {}", 42, 1.337);
    println!("Louis");

    x86_64::instructions::interrupts::int3();

    #[cfg(test)]
    test_main();

    println!("it did not crash");

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

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    os_rust::test_panic_handler(info)
}

tests! {
    testing_works {
        let one = 1;
        assert_eq!(1, one);
        assert_ne!(1, 4);
    }
}
