#![feature(llvm_asm)]
#![feature(core_intrinsics)]

#![no_std]
#![no_main]

extern crate avrd;

mod utils;
mod scheduler;

use core::intrinsics::volatile_store;
use avrd::atmega2560::*;
use utils::delay::*;

#[no_mangle]
pub extern fn main() {
  scheduler::run_scheduler();
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
  loop {}
}
