use rustabvm::Computer;
use rustabassembler::assemble;
use vmtranslator::translate;

fn main() {
    let vm = "
        function Main 0
        push constant 3
        push constant 4
        call Add 2
        function Add 0
        push argument 0
        push argument 1
        add
        return
    ";

    let asm = translate(vm);
    let binary = assemble(&asm);

    let mut computer = Computer::new(binary);
    computer.run(1000);
    println!("SP       = {}", computer.peek(0));
    println!("stack[0] = {}", computer.peek(256));
    println!("RAM[5]   = {}", computer.peek(5));   // temp 0
    println!("RAM[3]   = {}", computer.peek(3));   // pointer 0
}