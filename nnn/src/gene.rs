use std::rc::Rc;
use std::cell::RefCell;

use rand::random;

use config::Config;
use gene::{NodeType, NodeGene};

use crate::activations::ActivationFunctions;
use crate::aggregations::AggregationFunctions;

pub struct StandardNode {
    key: u64,
    typ: NodeType,
    activation_id: u8,
    aggregation_id: u8,
    config: Rc<RefCell<Config>>
}

impl StandardNode {
    pub fn new(key: u64, typ: NodeType, config: Rc<RefCell<Config>>) -> Self {
        StandardNode {
            key,
            typ,
            activation_id: 0,
            aggregation_id: 0,
            config
        }
    }

    pub fn mutate_function(&mut self) {
        let rand_act = (random::<f64>() / f64::MAX).abs();
        let rand_agg = (random::<f64>() / f64::MAX).abs();

        if rand_act < self.config.borrow().act_mut_chance() {
            self.activation_id = 
                random::<u8>() % ActivationFunctions::get_num_funcs();
        }

        if rand_agg < self.config.borrow().agg_mut_chance() {
            self.aggregation_id =
                random::<u8>() % AggregationFunctions::get_num_funcs();
        }
    }
}

impl NodeGene for StandardNode {
    fn get_key(&self) -> u64 {
        self.key
    }

    fn get_type(&self) -> NodeType {
        self.typ
    }

    fn activate(&self, val: f64) -> f64 {
        ActivationFunctions::activate(self.activation_id, val)
    }

    fn aggregate(&self, vals: Vec<f64>) -> f64 {
        AggregationFunctions::aggregate(self.aggregation_id, vals)
    }
}