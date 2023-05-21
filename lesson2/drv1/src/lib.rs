#![no_std]

use drv_common::{CallEntry, Driver};
use drv_macro::driver;

// #[used]
// #[link_section = ".init_calls"]
// static DRV1_ENTRY: CallEntry = CallEntry {
//     init_fn: drv1_init_fn,
// };

// fn drv1_init_fn() -> Driver<'static> {
//     Driver::info("uart", "ns16550a")
// }

driver! {
    drv_name: "drv0",
    name: "rtc",
    compatible: "google,goldfish-rtc",
}
