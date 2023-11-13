#![no_std]
#![no_main]

use core::panic::PanicInfo;
use os_rust::{exit_qemu, QemuExitCode};
use serial::{serial_print, serial_println};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_wrong_assert_fails();
    exit_qemu(QemuExitCode::Failed);

    #[allow(clippy::empty_loop)]
    loop {}
}

fn test_wrong_assert_fails() {
    serial_print!("test wrong assert fails\t");
    assert_eq!(0, 1);
    serial_print!("[ok]");
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("[ok]");
    exit_qemu(QemuExitCode::Success);
    loop {}
}
