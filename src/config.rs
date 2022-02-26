pub struct Config {
    innovation: u64
}

impl Config {
    pub fn new() -> Self {
        Config { innovation: 0 }
    }

    pub fn new_innovation(&mut self) -> u64 {
        let ret = self.innovation;
        self.innovation = self.innovation.checked_add(1)
            .unwrap("No innovations available anymore");
        ret
    }
}