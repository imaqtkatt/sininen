use bytecode::Bytecode;
use machine::Machine;
use value::{Constant, PrototypeBuilder};

mod bytecode;
mod machine;
mod stack;
mod value;

fn main() {
  // let main = PrototypeBuilder::new()
  //   .arity(0)
  //   .locals(0)
  //   .constant(value::Constant::number(42.))
  //   .emit(Bytecode::LoadConst { index: 0 })
  //   .emit(Bytecode::Closure { index: 0 })
  //   .emit(Bytecode::Call { arguments: 1 })
  //   .emit(Bytecode::Return)
  //   .prototype(
  //     PrototypeBuilder::new()
  //       .arity(1)
  //       .locals(1)
  //       .constant(value::Constant::number(1.))
  //       .emit(Bytecode::Load { index: 0 })
  //       .emit(Bytecode::LoadConst { index: 0 })
  //       .emit(Bytecode::Add)
  //       .emit(Bytecode::Return)
  //       .build(),
  //   )
  //   .build();

  // let main = PrototypeBuilder::new()
  //   .arity(0)
  //   .locals(1)
  //   .constant(Constant::number(41.))
  //   .constant(Constant::number(1.))
  //   .emit(Bytecode::LoadConst { index: 0 })
  //   .emit(Bytecode::LoadConst { index: 1 })
  //   .emit(Bytecode::Add)
  //   .emit(Bytecode::Return)
  //   .build();

  let main = PrototypeBuilder::new()
    .arity(0)
    .locals(1)
    .constant(Constant::number(41.))
    .constant(Constant::number(1.))
    .emit(Bytecode::LoadConst { index: 0 })
    .emit(Bytecode::Closure { index: 0 })
    .emit(Bytecode::Call { arguments: 1 })
    .emit(Bytecode::Store { index: 0 })
    .emit(Bytecode::LoadConst { index: 1 })
    .emit(Bytecode::Load { index: 0 })
    .emit(Bytecode::Call { arguments: 1 })
    .emit(Bytecode::Return)
    .prototype(
      PrototypeBuilder::new()
        .arity(2)
        .locals(2)
        .emit(Bytecode::Load { index: 0 })
        .emit(Bytecode::Load { index: 1 })
        .emit(Bytecode::Add)
        .emit(Bytecode::Return)
        .build(),
    )
    .build();

  let (mut machine, mut stack) = Machine::boot(main);
  match machine.run(&mut stack) {
    Ok(()) => {
      println!("stack = {stack:?}")
    }
    Err(msg) => eprintln!("{msg}"),
  }
}
