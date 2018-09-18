// TODO: Simple runner to test the library.  Should probably be removed from source control at some point.
#![no_main]
#![no_std]

#[macro_use]
extern crate cortex_m;
#[macro_use]
extern crate cortex_m_rt;
extern crate stm32l4;

extern crate stm32l433_hal;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}

entry!(main);

fn main() -> ! {
    let _y;
    let x = 42;
    _y = x;
    let w = 43;

    loop {}
}