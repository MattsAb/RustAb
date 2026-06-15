use rustabvm::Computer;
use rustabassembler::assemble;

fn main() {
    let source = "
        @0
        M=1
        @1
        M=0
        (LOOP)
        @0
        D=M
        @100
        D=D-A
        @END
        D;JGT
        @0
        D=M
        @1
        M=D+M
        @0
        M=M+1
        @LOOP
        0;JMP
        (END)
        @END
        0;JMP
    ";

    let program = assemble(source);
    let mut computer = Computer::new(program);
    computer.run(3000);

    println!("i   = {}", computer.peek(0));
    println!("sum = {}", computer.peek(1));
}