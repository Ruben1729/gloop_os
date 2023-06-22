#![test_runner(gloop_os::test_runner)]

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    gloop_os::test_panic_handler(info)
}

use gloop_os::println;

#[test_case]
fn test_println() {
    println!("test_println output");
}
