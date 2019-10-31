#![no_main]
#![no_std]

extern crate panic_halt;
extern crate cortex_m;
extern crate cortex_m_rt;
extern crate cortex_m_semihosting;
extern crate stm32l4;

use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;

#[entry]
fn main() -> ! {
    let mut count: u32 = 0;
    loop {
        hprintln!("Count: {}", count).unwrap();
        count += 1;
    }
}