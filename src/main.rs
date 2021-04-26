#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(ishtar::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use ishtar::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello Ishtar{}", "!");

    ishtar::init();

    #[cfg(test)]
    test_main();

    println!("nothing special!");
    ishtar::hlt_loop();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    ishtar::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    ishtar::test_panic_handler(info)
}