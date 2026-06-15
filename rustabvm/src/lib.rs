mod register;
mod pc;
mod alu;


use register::Register;
use pc::Pc;
use alu::Alu;

#[derive(Debug, Default)]
struct Cpu {
    a: Register,
    d: Register,
    alu: Alu,
    pc: Pc,
}

impl Cpu {
    /// One clock cycle.
    /// - instruction: 16-bit value from ROM
    /// - in_m:        value from RAM[A]
    /// Returns (out_m, write_m, address_m, pc)
    fn tick(&mut self, instruction: u16, in_m: u16) -> (u16, bool, u16, u16) {

        let is_c = (instruction & 0x8000) != 0; // MSB=1 → C-instruction

        // --- Decode C-instruction fields ---
        let a_bit = (instruction & 0x1000) != 0; // comp: use M or A as ALU y-input
        
        let c1 = (instruction & 0x0800) != 0; // zx
        let c2 = (instruction & 0x0400) != 0; // nx
        let c3 = (instruction & 0x0200) != 0; // zy
        let c4 = (instruction & 0x0100) != 0; // ny
        let c5 = (instruction & 0x0080) != 0; // f
        let c6 = (instruction & 0x0040) != 0; // no

        let d1 = (instruction & 0x0020) != 0; // dest: A
        let d2 = (instruction & 0x0010) != 0; // dest: D
        let d3 = (instruction & 0x0008) != 0; // dest: M

        let j1 = (instruction & 0x0004) != 0; // jump if ng
        let j2 = (instruction & 0x0002) != 0; // jump if zr
        let j3 = (instruction & 0x0001) != 0; // jump if positive

        // --- A register ---
        // A-instruction: load the immediate value
        // C-instruction: load ALU output only if d1 is set
        let a_input = if is_c { self.alu.out } else { instruction };
        let load_a  = !is_c || (is_c && d1);
        self.a.tick(a_input, load_a);


        self.alu.x  = self.d.value;
        self.alu.y  = if a_bit { in_m } else { self.a.value };
        self.alu.zx = c1;
        self.alu.nx = c2;
        self.alu.zy = c3;
        self.alu.ny = c4;
        self.alu.f  = c5;
        self.alu.no = c6;
        self.alu.compute();


        self.d.tick(self.alu.out, is_c && d2);


        let neg = self.alu.ng;
        let zer = self.alu.zr;
        let pos = !neg && !zer;

        let jump = is_c && (
            (j1 && neg) ||
            (j2 && zer) ||
            (j3 && pos)
        );

        
        let pc_out = self.pc.tick(self.a.value, true, jump, false);

        // --- Outputs ---
        let out_m     = self.alu.out;
        let write_m   = is_c && d3;
        let address_m = self.a.value;

        (out_m, write_m, address_m, pc_out)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn cpu_a_instruction() {
        let mut cpu = Cpu::default();
        cpu.tick(10, 0);
        assert_eq!(cpu.a.value, 10);
    }

    #[test]
    fn cpu_d_equals_a() {
        let mut cpu = Cpu::default();
        cpu.tick(17, 0);           // @17  → A=17
        cpu.tick(0xEC10, 0);       // D=A  → D=17
        assert_eq!(cpu.d.value, 17);
    }

    #[test]
    fn cpu_write_to_memory() {
        let mut cpu = Cpu::default();
        cpu.tick(10, 0);            // @10      → A=10
        cpu.tick(0xEC10, 0);        // D=A      → D=10
        cpu.tick(5, 0);             // @5       → A=5
        let (out, write, addr, _) = cpu.tick(0xE308, 0); // M=D
        assert!(write);
        assert_eq!(addr, 5);
        assert_eq!(out, 10);
    }

    #[test]
    fn cpu_jump() {
        let mut cpu = Cpu::default();
        cpu.tick(100, 0);           // @100     → A=100
        // D=0 by default, JEQ (j2=1): 1 1 1 0 1 0 1 0 1 0 0 0 0 0 1 0 = 0xEA82
        let (_, _, _, pc) = cpu.tick(0xEA82, 0);
        assert_eq!(cpu.pc.value, 100); // jumped to 100
    }
}