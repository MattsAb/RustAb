#[derive(Debug, Default)]
pub struct Pc {
    pub value: u16,
}

impl Pc {
    pub fn tick(&mut self, input: u16, load: bool, inc: bool, reset: bool) -> u16 {

        if reset {
            self.value = 0;
        } else if load {
            self.value = input;
        } else if inc {
            self.value = self.value.wrapping_add(1);
        }
        self.value
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn load_value() {
        let mut pc = Pc::default();
        pc.tick(10,true,false,false);
        assert_eq!(pc.value, 10);
    }

    #[test]
    fn reset_value() {
        let mut pc = Pc::default();
        pc.tick(10,true,false,false);
        assert_eq!(pc.value, 10);
        pc.tick(5,false,false,true);
        assert_eq!(pc.value, 0);
    }

    #[test]
    fn increment_value() {
        let mut pc = Pc::default();
        pc.tick(0,true,false,false);
        assert_eq!(pc.value, 0);
        pc.tick(5,false,true,false);
        assert_eq!(pc.value, 1);
    }

}