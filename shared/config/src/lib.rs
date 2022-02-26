pub struct Config {
    innovation: u64,
    aggregate_mutation_chance: f64,
    activation_mutation_chance: f64,
}

impl Config {
    pub fn new(agg_chance: f64, act_chance: f64) -> Self {
        Config { 
            innovation: 0,
            aggregate_mutation_chance: agg_chance,
            activation_mutation_chance: act_chance
        }
    }

    pub fn new_innovation(&mut self) -> u64 {
        let ret = self.innovation;
        self.innovation = self.innovation.checked_add(1)
            .expect("No innovations available anymore");
        ret
    }

    pub fn agg_mut_chance(&self) -> f64 {
        self.aggregate_mutation_chance
    }

    pub fn act_mut_chance(&self) -> f64 {
        self.activation_mutation_chance
    }
}