use test::Bencher;
use super::executor::Executor;
use super::program::Program;
use super::basic_block::BasicBlock;
use super::opcode::OpCode;
use super::function::Function;

#[bench]
fn bench_invoke(b: &mut Bencher) {
    let mut executor = Executor::new();
    let program = Program::from_functions(vec! [
        Function::from_basic_blocks(vec! [
            BasicBlock::from_opcodes(vec! [
                { OpCode::Return }
            ])
        ])
    ]);
    b.iter(|| {
        executor.eval_program(&program, 0)
    });
}