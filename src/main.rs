#![no_main]
#![no_std]
#![feature(custom_test_frameworks)]
#![test_runner(os_primary::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use os_primary::{init, hlt_loop, println};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("\nHello World{}", "!");

    init();

    #[cfg(test)]
    test_main();

    hlt_loop();
}

#[cfg(not(test))] // new attribute
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    os_primary::test_panic_handler(info)
}
