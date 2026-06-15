mod register;
mod pc;
mod alu;
mod ram;
mod rom;
mod cpu;

use ram::Ram;
use rom::Rom;
use cpu::Cpu;

pub struct Computer {
    pub cpu: Cpu,
    pub ram: Ram,
    pub rom: Rom,
}

impl Computer {

    pub fn new(program: Vec<u16>) -> Self {
        Computer {
            cpu: Cpu::default(),
            ram: Ram::new(16384),
            rom: Rom::new(program),
        }
    }

    pub fn tick(&mut self) {
        let pc = self.cpu.pc.value as usize;
        let instruction = self.rom.read(pc);
        let in_m = self.ram.tick(0, self.cpu.a.value as usize, false);
        println!("in_m: {}", in_m);


        let (out_m, write_m, address_m, _) = self.cpu.tick(instruction, in_m);
        println!("out_m: {}, write_m: {}, adress_m: {}", out_m, write_m,address_m);
        if write_m {
            self.ram.tick(out_m, address_m as usize, true);
        }
    }

    pub fn run(&mut self, cycles: usize) {
        for _ in 0..cycles {
            self.tick();
        }
    }

    pub fn peek(&mut self, address: usize) -> u16 {
        self.ram.tick(0, address, false)
    }

}