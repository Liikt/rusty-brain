use std::rc::Rc;
use std::cell::RefCell;

use rand::random;

use config::Config;
use gene::{NodeType, NodeGene};

use crate::activations::ActivationFunctions;
use crate::aggregations::AggregationFunctions;

#[derive(Debug, Clone)]
pub struct StandardNode {
    key: u64,
    typ: NodeType,
    activation_id: u8,
    aggregation_id: u8,
}

impl StandardNode {
    pub fn new(key: u64, typ: NodeType) -> Self {
        StandardNode {
            key,
            typ,
            activation_id: 0,
            aggregation_id: 0,
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