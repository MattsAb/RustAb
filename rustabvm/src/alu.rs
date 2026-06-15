#[derive(Debug, Default)]
pub struct Alu {
    pub x: u16,
    pub y: u16,

    pub zx: bool,
    pub nx: bool,
    pub zy: bool,
    pub ny: bool,
    pub f: bool,
    pub no: bool,

    pub out: u16,
    pub zr: bool,
    pub ng: bool,
}

impl Alu {
    pub fn compute(&mut self) {

        let mut x = self.x;
        if self.zx {x = 0;}
        if self.nx {x = !x;}

        let mut y = self.y;
        if self.zy {y = 0;}
        if self.ny {y = !y;}

        let mut out: u16 = if self.f {
            x.wrapping_add(y)
        } else {
            x & y
        };

        if self.no {out = !out;}

        self.out = out;
        self.zr  = out == 0;
        self.ng  = (out & 0x8000) != 0;

    }

}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn alu_zero() {
        let mut alu = Alu::default();
        alu.x = 999; alu.y = 999;
        alu.zx = true;  alu.nx = false;
        alu.zy = true;  alu.ny = false;
        alu.f  = true;  alu.no = false;
        alu.compute();
        assert_eq!(alu.out, 0);
        assert!(alu.zr);
        assert!(!alu.ng);
    }
    #[test]
    fn alu_add() {
        let mut alu = Alu::default();
        alu.x = 10; alu.y = 5;
        alu.zx = false;  alu.nx = false;
        alu.zy = false;  alu.ny = false;
        alu.f  = true;  alu.no = false;
        alu.compute();
        assert_eq!(alu.out, 15);
        assert!(!alu.zr);
        assert!(!alu.ng);
    }

}