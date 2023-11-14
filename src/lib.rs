#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks, abi_x86_interrupt)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

pub mod gdt;
pub mod interrupts;
pub mod vga_buffer;

use core::panic::PanicInfo;
use serial::serial_println;

/// This function is run on startup regardless of whether it’s in testing mode or not
pub fn init() {
    gdt::init();
    interrupts::init_idt();
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    init();
    test_main();

    #[allow(clippy::empty_loop)]
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info);
}

pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);

    #[allow(clippy::empty_loop)]
    loop {}
}

tests! {
    testing_works {
        let one = 1;
        assert_eq!(1, one);
        assert_ne!(1, 4);
    }
}

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
                serial::serial_print!("test ");

                let mut it = stringify!($name).split(' ').peekable();

                while let Some(word) = it.next()  {
                    serial::serial_print!("{}", word);

                    if !it.peek().is_none() {
                        serial::serial_print!(" ");
                    }
                }
                serial::serial_print!("...\t");
                $body
                serial::serial_println!("[ok]");
            }
        )*
    };
}
