#[derive(Debug, Default)]
pub struct Register {
    pub value: u16
}

impl Register {
    pub fn tick(&mut self, input: u16, load: bool) -> u16 {
        if load {
            self.value = input;
        } 
        self.value
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn change_value() {
        let mut register = Register::default();
        register.tick(10,true);
        assert_eq!(register.value, 10);
    }

    #[test]
    fn keep_value() {
        let mut register = Register::default();
        register.tick(10,true);
        assert_eq!(register.value, 10);
        register.tick(15,false);
        assert_eq!(register.value, 10);
    }

}