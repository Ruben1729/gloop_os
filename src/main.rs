#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![feature(custom_test_frameworks)]
#![test_runner(gloop_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use gloop_os::println;
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    gloop_os::init();

    // trigger a page fault
    unsafe {
        *(0xdeadbeef as *mut u8) = 42;
    };

    // as before
    #[cfg(test)]
    test_main();

    println!("It did not crash!");
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
    gloop_os::test_panic_handler(info)
}

