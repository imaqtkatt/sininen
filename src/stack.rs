use crate::value::Value;

#[derive(Debug)]
pub struct Stack {
  stack_base: usize,
  local_base: usize,
  values: Vec<Value>,
}

#[derive(Clone, Copy)]
pub struct FrameInfo {
  stack_base: usize,
  local_base: usize,
}

impl Stack {
  pub fn new_with_local_size(local_size: usize) -> Self {
    Self {
      stack_base: local_size,
      local_base: 0,
      values: vec![Value::default(); local_size],
    }
  }

  pub fn push_frame(&mut self, locals_size: usize) -> FrameInfo {
    let new_local_base = self.values.len();
    let new_stack_base = new_local_base + locals_size;

    self.values.resize(new_stack_base, Value::default());

    FrameInfo {
      stack_base: std::mem::replace(&mut self.stack_base, new_stack_base),
      local_base: std::mem::replace(&mut self.local_base, new_local_base),
    }
  }

  pub fn pop_frame(
    &mut self,
    FrameInfo {
      stack_base,
      local_base,
    }: FrameInfo,
  ) {
    self.values.drain(self.local_base..self.stack_base);
    self.stack_base = stack_base;
    self.local_base = local_base;
  }

  #[inline(always)]
  pub fn load_local(&mut self, index: usize) -> Value {
    self.values[self.local_base + index].clone()
  }

  #[inline(always)]
  pub fn store_local(&mut self, index: usize, value: Value) {
    self.values[dbg!(self.local_base + index)] = value;
  }

  #[inline(always)]
  pub fn push(&mut self, value: Value) {
    self.values.push(value);
  }

  #[inline(always)]
  pub fn pop(&mut self) -> Option<Value> {
    if self.check_underflow(1) {
      None
    } else {
      Some(self.values.pop().unwrap())
    }
  }

  #[inline(always)]
  pub fn pop_many(&mut self, count: usize) -> Option<Vec<Value>> {
    if self.check_underflow(count) {
      None
    } else {
      Some(self.values.split_off(self.values.len() - count))
    }
  }

  #[inline(always)]
  fn check_underflow(&self, count: usize) -> bool {
    self.values.len() - count < self.stack_base
  }
}

#[cfg(test)]
mod test {
  use super::Stack;

  #[test]
  fn test_pop() {
    let mut stack = Stack::new_with_local_size(1);
    stack.store_local(0, crate::value::Value::Boolean(false));

    _ = stack.push_frame(0);

    stack.push(crate::value::Value::Boolean(true));
    _ = stack.pop().unwrap();

    assert!(stack.pop().is_none());
  }
}
