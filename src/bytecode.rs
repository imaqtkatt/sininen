#[derive(Clone, Copy, Debug)]
pub enum Bytecode {
  Return,
  Load { index: u16 },
  Store { index: u16 },
  Closure { index: u32 },
  Call { arguments: u8 },
  LoadConst { index: u16 },
  Add,
}
