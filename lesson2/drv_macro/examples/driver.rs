// use drv_macro::driver;

// use drv_common::{CallEntry, Driver};

// driver! {
//     drv_name: "drv0",
//     name: "rtc",
//     compatible: "google,goldfish-rtc",
// }

fn main() {}

// #![no_std]

// use drv_common::{CallEntry, Driver};

// #[used]
// #[link_section = ".init_calls"]
// static DRV0_ENTRY: CallEntry = CallEntry {
//     init_fn: drv0_init_fn,
// };

// fn drv0_init_fn() -> Driver<'static> {
//     Driver::info("rtc", "google,goldfish-rtc")
// }
