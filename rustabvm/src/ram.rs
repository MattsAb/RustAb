#[derive(Debug)]
pub struct Ram {
    pub data: Vec<u16>,
}

impl Ram {
    pub fn new(size: usize) -> Self {
        Ram {
            data: vec![0; size],
        }
    }

    pub fn tick(&mut self, input: u16, address: usize, load: bool) -> u16 {
        if load {
            self.data[address] = input;
        }
        self.data[address]
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn change_value() {
        let mut ram = Ram::new(8);
        ram.tick(5,0,true);
        assert_eq!(ram.data[0], 5);
    }

    #[test]
    fn keep_value() {
        let mut ram = Ram::new(8);
        ram.tick(5,0,true);
        ram.tick(10,0,false);
        assert_eq!(ram.data[0], 5);
    }

}