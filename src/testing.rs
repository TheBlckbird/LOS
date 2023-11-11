#[cfg(test)]
use crate::exit_qemu::{exit_qemu, QemuExitCode};
#[cfg(test)]
use crate::serial_println;

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
