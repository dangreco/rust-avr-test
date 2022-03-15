#![feature(llvm_asm)]
#![feature(core_intrinsics)]

#![no_std]
#![no_main]

extern crate avrd;
use core::intrinsics::volatile_store;
use avr_delay::delay_ms;
use avrd::atmega2560::*;

fn delay(ms: u32) {
  for _ in 0..(ms / 16) { // idk
    delay_ms(100)
  }
}

#[no_mangle]
pub extern fn main() {
  let mut out: u8 = 0x00;
  unsafe { volatile_store(DDRB, 0xff) }
   loop {
    out = out ^ 0xff;
    unsafe { volatile_store(PORTB, out) }
    delay(1000);
  }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
  loop {}
}
