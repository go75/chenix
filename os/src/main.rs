#![no_main]
#![no_std]
#![feature(panic_info_message)]
mod lang_items;
mod sbi;

#[macro_use]
mod console;

use core::arch::global_asm;
global_asm!(include_str!("entry.asm"));

#[no_mangle]
pub fn chenix_main() {
    clean_bss();
    println!("Hello chenix!");
    loop{};
}

fn clean_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| {
        unsafe {
            (a as *mut u8).write_volatile(0)
        }
    })
}