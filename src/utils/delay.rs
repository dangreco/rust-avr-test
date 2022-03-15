pub fn delay(cycles: u32)
{
  let outer = cycles / 0xffff;
  let last = ((cycles % 0xffff)+1) as u16;

  for _ in 0..outer {
    unsafe {
      llvm_asm!(
        "1: sbiw $0,1
            brne 1b
        "
        :: "w" (0)
        ::
      );
    }
  }

  unsafe {
    llvm_asm!(
      "1: sbiw $0,1
          brne 1b
      "
      :: "w" (last)
      ::
    );
  }
}

pub fn delay_us(us: u32, freq_hz: u32)
{
  let ps = us * 1000;
  let ps_lp = 1000000000 / (freq_hz / 4);
  let cycles = (ps / ps_lp) as u32;
  delay(cycles);
}

pub fn delay_ms(ms: u32, freq_hz: u32)
{
  let us = ms * 1000;
  delay_us(us, freq_hz);
}