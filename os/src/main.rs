// os/src/main.rs
#![no_main] // inidcate no main
#![no_std] // indicate no std library
#![feature(panic_info_message)]
#[macro_use]
mod console;
mod lang_items; // mod: delcares a module in rust
mod sbi;
// global_asm: embedded assembly into rust code
use core::arch::global_asm;


/* (entry) & (rust_main) section */
global_asm!(include_str!("entry.asm"));
#[no_mangle]
pub fn rust_main() -> ! { // return type ! means `never`
                        // function will never return to its caller
    clear_bss();
    // console_putchar('O' as usize);
    // console_putchar('K' as usize);
    println!("Hello, world!");
    panic!("Shutdown machine!");
}

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| {
        unsafe { (a as *mut u8).write_volatile(0) }
    });
}