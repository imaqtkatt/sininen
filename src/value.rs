use std::cell::RefCell;

use crate::bytecode::Bytecode;

#[derive(Clone)]
pub enum Value {
  /// f64 bits
  Number(u64),
  Boolean(bool),
  Closure(std::rc::Rc<Prototype>),
  Partial(std::rc::Rc<RefCell<Partial>>),
}

impl std::fmt::Debug for Value {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Number(arg0) => write!(f, "Number({})", f64::from_bits(*arg0)),
      Self::Boolean(arg0) => write!(f, "Boolean({})", arg0),
      Self::Closure(arg0) => write!(f, "<closure@{:?}>", std::rc::Rc::as_ptr(arg0)),
      Self::Partial(arg0) => write!(f, "<partial@{:?}>", std::rc::Rc::as_ptr(arg0)),
    }
  }
}

impl Default for Value {
  fn default() -> Self {
    Value::Number(f64::to_bits(0.))
  }
}

#[derive(Clone, Debug)]
pub struct Prototype {
  pub arity: u8,
  pub locals: u16,
  pub constant_pool: Vec<Constant>,
  pub prototypes: Vec<Prototype>,
  pub bytecode: Vec<Bytecode>,
}

pub struct PrototypeBuilder {
  arity: u8,
  locals: u16,
  constant_pool: Vec<Constant>,
  prototypes: Vec<Prototype>,
  bytecode: Vec<Bytecode>,
}

impl PrototypeBuilder {
  pub fn new() -> Self {
    Self {
      arity: 0,
      locals: 0,
      constant_pool: vec![],
      prototypes: vec![],
      bytecode: vec![],
    }
  }

  pub fn emit(mut self, bytecode: Bytecode) -> Self {
    self.bytecode.push(bytecode);
    self
  }

  pub fn arity(self, arity: u8) -> Self {
    Self { arity, ..self }
  }

  pub fn locals(self, locals: u16) -> Self {
    Self { locals, ..self }
  }

  pub fn constant(mut self, constant: Constant) -> Self {
    self.constant_pool.push(constant);
    self
  }

  pub fn prototype(mut self, proto: Prototype) -> Self {
    self.prototypes.push(proto);
    self
  }

  pub fn build(self) -> Prototype {
    Prototype {
      arity: self.arity,
      locals: self.locals,
      constant_pool: self.constant_pool,
      prototypes: self.prototypes,
      bytecode: self.bytecode,
    }
  }
}

#[derive(Clone, Copy, Debug)]
pub enum Constant {
  Number(u64),
}

impl Constant {
  pub fn number(n: f64) -> Self {
    Self::Number(f64::to_bits(n))
  }
}

#[derive(Clone, Debug)]
pub struct Partial {
  pub arity: u8,
  pub applied: u8,
  pub prototype: std::rc::Rc<Prototype>,
  pub applied_values: Vec<Value>,
}
