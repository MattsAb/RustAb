pub struct Rom {
    pub data: Vec<u16>,
}

impl Rom {
    pub fn new(program: Vec<u16>) -> Self {
        Rom { data: program }
    }

    pub fn read(&self, address: usize) -> u16 {
        if address >= self.data.len() {
            return 0;
        }
        self.data[address]
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn read_value() {
        let rom = Rom::new(vec![0,0,0,0,0,0,0,0]);
        assert_eq!(rom.read(5), 0);
    }

}