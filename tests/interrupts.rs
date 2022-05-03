#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(os_primary::test_runner)]
#![reexport_test_harness_main = "test_main"]

use os_primary::interrupts;
use core::panic::PanicInfo;

/// Entry point for `cargo test`
#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    interrupts::init_idt();      // new
    test_main();
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    os_primary::test_panic_handler(info)
}

#[test_case]
fn test_breakpoint_exception() {
    // invoke a breakpoint exception
    x86_64::instructions::interrupts::int3();
}