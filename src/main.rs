//! doc

#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![deny(missing_docs)]
#![deny(warnings)]

use core::arch::global_asm;
use log::*;

#[macro_use]
mod console;
mod lang_items;
mod logging;
mod sbi;


global_asm!(include_str!("entry.asm"));

/// entry point of os
#[no_mangle]
pub fn rust_main() -> ! {
    extern "C" {
        fn stext();
        fn etext();
        fn srodata();
        fn erodata();
        fn sdata();
        fn edata();
        fn sbss();
        fn ebss();
        fn boot_stack_lower_bound();
        fn boot_stack_top();
    }


    clera_bss();
    logging::init();
    println!("[kernel] Hello, world");
    trace!(
        "[kernel] .text [{:#x}, {:#x})",
        stext as usize,
        etext as usize
    );
    debug!(
        "[kernel] .rodata [{:#x}, {:#x})",
        srodata as usize, erodata as usize
    );
    info!(
        "[kernel] .data [{:#x}, {:#x})",
        sdata as usize, edata as usize
    );
    warn!(
        "[kernel] boot_stack top=bottom={:#x}, lower_bound={:#x}",
        boot_stack_top as usize, boot_stack_lower_bound as usize
    );
    error!("[kernel] .bss [{:#x}, {:#x})", sbss as usize, ebss as usize);
    sbi::console_putchar('O' as usize);
    sbi::console_putchar('K' as usize);
    sbi::console_putchar('\n' as usize);
    println!("Hello, World");
    print!("hello, world \n");
    println!("Hello, World");

    sbi::shutdown(false);
   //  loop {sbi::console_putchar('O' as usize);
   //  sbi::console_putchar('K' as usize);}
}

fn clera_bss(){
    extern "C"{
       fn sbss();
       fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| unsafe{(a as *mut u8).write_volatile(0) }) 
}