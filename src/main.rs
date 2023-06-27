#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![feature(custom_test_frameworks)]
#![feature(restricted_std)]
#![test_runner(gloop_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::{BootInfo, entry_point};
use gloop_os::task::{executor::Executor, keyboard, Task};

entry_point!(kernel_main);

extern crate alloc;
extern crate std;

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use gloop_os::allocator;
    use gloop_os::memory::{self, BootInfoFrameAllocator};
    use x86_64::VirtAddr;

    gloop_os::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

    #[cfg(test)]
    test_main();

    let mut executor = Executor::new();
    executor.spawn(Task::new(keyboard::print_keypresses()));
    executor.run();
}

// #[cfg(not(test))]
// #[panic_handler]
// fn panic(info: &PanicInfo) -> ! {
//     println!("{}", info);
//     halt();
// }

// #[cfg(test)]
// #[panic_handler]
// fn panic(info: &PanicInfo) -> ! {
//     gloop_os::test_panic_handler(info)
// }

