pub mod pcb;

use core::ptr::{null};

use pcb::PCB;

static mut PCB_TABLE: [PCB; 32] = [PCB {
  state: 0,
  pid: 0,
  pc: null(),
}; 32];

pub fn schedule(pc: *const u8) -> bool
{
  unsafe {
    for i in 0..32 {
      if PCB_TABLE[i].state == 0 {
        PCB_TABLE[i].pc = pc as *mut u8;
        return true;
      }
    }
  }

  false
}

fn sstate()
{

}

fn lstate()
{

}

pub fn run_scheduler()
{
  loop {

  }
}