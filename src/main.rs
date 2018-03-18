// #![deny(missing_docs)]
// #![deny(warnings)]
#![no_std]
#![feature(used)]
// #![feature(lang_items)]
// #[allow(unused_variables)]


// version = "0.2.0", default-features = false
// extern crate cast;
extern crate bare_metal;
extern crate vcell;
extern crate cortex_m;
extern crate nrf52840;
#[cfg(feature = "rt")]
extern crate cortex_m_rt;

// use core::u16;

// use cast::{u16, u32};
// use cortex_m::asm;
// use cortex_m::peripheral::Peripherals;
use nrf52840::{Peripherals, P0};

#[inline(never)]
fn main() {
    // cortex_m::interrupt::free(
    //     |cs| {
    //         let mut peripherals = Peripherals::take().unwrap();
    //         let poblock: P0 = peripherals.P0;
    //         poblock.out.modify(|_,w| unsafe {w.bits(0b1000000000000000000)});
    //     });

    let mut peripherals = Peripherals::take().unwrap();
    let poblock: P0 = peripherals.P0;
    poblock.out.modify(|_,w| unsafe {w.bits(0b1000000000000000000)});
}