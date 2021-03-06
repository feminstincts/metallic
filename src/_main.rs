#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

//test
#![feature(custom_test_frameworks)]
#![test_runner(metallic::test_runner)]
#![reexport_test_harness_main = "test_main"]

use metallic::println;
use core::panic::PanicInfo;

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {

	println!("\tWelcome{} to {}", ",", "metallic");    
    metallic::init();

	// trigger a page fault
    // unsafe {
    //     *(0xdeadbeef as *mut u64) = 42;
    // };

	// invoke a breakpoint exception
    // x86_64::instructions::interrupts::int3();

    // fn stack_overflow() {
    //     stack_overflow(); // for each recursion, the return address is pushed
    // }

    // uncomment line below to trigger a stack overflow
    // stack_overflow();

	#[cfg(test)]
	test_main();

    println!("\n It did not crash!");
	println!("\n -> ");
    metallic::hlt_loop();
    // loop {
	    // deadlock and race condition
		// use metallic::print;
        // print!("-");      	
    // }
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    metallic::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    metallic::test_panic_handler(info)
}