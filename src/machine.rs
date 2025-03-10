use std::cell::RefCell;

use crate::{
  stack::{FrameInfo, Stack},
  value::{Partial, Prototype},
};

pub struct Machine {
  ip: usize,
  prototype: std::rc::Rc<Prototype>,
  stack_frame: Vec<Frame>,
}

struct Frame {
  ip: usize,
  frame_info: FrameInfo,
  prototype: std::rc::Rc<Prototype>,
}

#[derive(Clone, Copy)]
pub enum Cont {
  Halt,
  Continue,
  Error(&'static str),
}

impl Machine {
  pub fn boot(proto: Prototype) -> (Self, Stack) {
    let stack = Stack::new_with_local_size(proto.locals as usize);
    (
      Self {
        ip: 0,
        prototype: std::rc::Rc::new(proto),
        stack_frame: vec![],
      },
      stack,
    )
  }

  pub fn run(&mut self, stack: &mut Stack) -> Result<(), &'static str> {
    loop {
      match self.step(stack) {
        Cont::Halt => break Ok(()),
        Cont::Continue => continue,
        Cont::Error(msg) => break Err(msg),
      }
    }
  }

  fn step(&mut self, stack: &mut Stack) -> Cont {
    let instruction = {
      let instruction = self.prototype.bytecode[self.ip];
      self.ip += 1;
      instruction
    };

    dbg!(&instruction);

    match instruction {
      crate::bytecode::Bytecode::Return if self.stack_frame.is_empty() => Cont::Halt,
      crate::bytecode::Bytecode::Return => {
        let frame = self.stack_frame.pop().unwrap();

        stack.pop_frame(frame.frame_info);
        self.ip = frame.ip;
        self.prototype = frame.prototype;

        Cont::Continue
      }
      crate::bytecode::Bytecode::Load { index } => {
        let value = stack.load_local(index as usize);
        stack.push(value);
        Cont::Continue
      }
      crate::bytecode::Bytecode::Store { index } => {
        let value = stack.pop().expect("underflow");
        stack.store_local(index as usize, value);
        Cont::Continue
      }
      crate::bytecode::Bytecode::Closure { index } => {
        let proto = self.prototype.prototypes[index as usize].clone();

        stack.push(crate::value::Value::Closure(std::rc::Rc::new(proto)));

        Cont::Continue
      }
      crate::bytecode::Bytecode::Call { arguments } => {
        let callee = stack.pop().expect("underflow");
        let mut args = stack.pop_many(arguments as usize).expect("underflow");

        match callee {
          crate::value::Value::Closure(closure) => {
            let arity = closure.arity;

            match arguments.cmp(&arity) {
              std::cmp::Ordering::Less => {
                let partial = Partial {
                  arity,
                  applied: args.len() as u8,
                  prototype: closure.clone(),
                  applied_values: args,
                };

                stack.push(crate::value::Value::Partial(std::rc::Rc::new(
                  RefCell::new(partial),
                )));

                Cont::Continue
              }
              std::cmp::Ordering::Equal => {
                let frame_info = stack.push_frame(closure.locals as usize);

                for index in (0..arity).rev() {
                  let value = args.pop().expect("");
                  stack.store_local(index as usize, value);
                }

                self.stack_frame.push(Frame {
                  ip: std::mem::replace(&mut self.ip, 0),
                  frame_info,
                  prototype: std::mem::replace(&mut self.prototype, closure.clone()),
                });

                Cont::Continue
              }
              std::cmp::Ordering::Greater => Cont::Error("arity error"),
            }
          }
          crate::value::Value::Partial(ref partial) => {
            let mut partial_borrow = partial.borrow_mut();
            let arity = partial_borrow.arity;

            let applied = partial_borrow.applied;
            let new_applied = applied + arguments;

            match new_applied.cmp(&arity) {
              std::cmp::Ordering::Less => {
                partial_borrow.applied_values.extend(args);
                partial_borrow.applied = new_applied;

                stack.push(crate::value::Value::Partial(partial.clone()));

                Cont::Continue
              }
              std::cmp::Ordering::Equal => {
                // this is so ugly
                partial_borrow.applied_values.extend(args);
                let mut args = partial_borrow.applied_values.clone();
                let frame_info = stack.push_frame(partial_borrow.prototype.locals as usize);

                for index in (0..arity).rev() {
                  let value = args.pop().expect("");
                  stack.store_local(index as usize, value);
                }

                self.stack_frame.push(Frame {
                  ip: std::mem::replace(&mut self.ip, 0),
                  frame_info,
                  prototype: std::mem::replace(
                    &mut self.prototype,
                    partial_borrow.prototype.clone(),
                  ),
                });

                Cont::Continue
              }
              std::cmp::Ordering::Greater => Cont::Error("arity error"),
            }
          }
          v => panic!("callee type error: {v:?}"),
        }
      }
      crate::bytecode::Bytecode::LoadConst { index } => {
        let value = self.prototype.constant_pool[index as usize];

        let value = match value {
          crate::value::Constant::Number(n) => crate::value::Value::Number(n),
        };

        stack.push(value);

        Cont::Continue
      }
      crate::bytecode::Bytecode::Add => {
        let rhs = stack.pop().expect("underflow");
        let lhs = stack.pop().expect("underflow");

        match (lhs, rhs) {
          (crate::value::Value::Number(x), crate::value::Value::Number(y)) => {
            let x = f64::from_bits(x);
            let y = f64::from_bits(y);
            let ans = x + y;

            stack.push(crate::value::Value::Number(f64::to_bits(ans)));

            Cont::Continue
          }
          _ => Cont::Error("type error"),
        }
      }
    }
  }
}
