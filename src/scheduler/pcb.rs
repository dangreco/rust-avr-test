#[derive(Clone, Copy)]
pub struct PCB {
  pub state: u8,
  pub pid: u8,
  pub pc: *const u8,
}