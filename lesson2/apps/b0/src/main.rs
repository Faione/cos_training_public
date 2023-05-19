#![no_std]
#![no_main]

use drv0 as _;
use drv1 as _;

use drv_common::CallEntry;

#[no_mangle]
fn main() {
    libos::init();

    libos::println!("\n[ArceOS Tutorial]: B0\n");
    verify();
}

/* Todo: Implement it */
fn traverse_drivers() {
    // libos::println!("\n!!! Fix it !!!\n");

    // 读取符号
    extern "C" {
        fn init_calls_start();
        fn init_calls_end();
    }

    // 直接获取符号的地址
    let (start, end) = unsafe { (init_calls_start as usize, init_calls_end as usize) };

    // Parse range of init_calls by calling C function.
    display_initcalls_range(start, end);

    // For each driver, display name & compatible
    let call_entries = unsafe {
        core::slice::from_raw_parts(
            start as *const CallEntry,
            (end - start) / core::mem::size_of::<CallEntry>(),
        )
    };

    call_entries.into_iter().for_each(|entry| {
        let drv = (entry.init_fn)();
        display_drv_info(drv.name, drv.compatible);
    });
}

fn display_initcalls_range(start: usize, end: usize) {
    libos::println!("init calls range: 0x{:X} ~ 0x{:X}\n", start, end);
}

fn display_drv_info(name: &str, compatible: &str) {
    libos::println!("Found driver '{}': compatible '{}'", name, compatible);
}

fn verify() {
    traverse_drivers();

    libos::println!("\nResult: Okay!");
}
