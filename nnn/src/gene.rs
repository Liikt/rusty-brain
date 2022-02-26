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
    bias: f64,
    weight: f64,
    activation_id: u8,
    aggregation_id: u8,
}

impl StandardNode {
    pub fn new(key: u64, typ: NodeType) -> Self {
        StandardNode {
            key,
            typ,
            bias: 0.0,
            weight: 1.0,
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

    fn get_bias(&self) -> f64 {
        return self.bias
    }

    fn get_weight(&self) -> f64 {
        self.weight
    }

    fn activate(&self, val: f64) -> f64 {
        ActivationFunctions::activate(self.activation_id, val)
    }

    fn aggregate(&self, vals: Vec<f64>) -> f64 {
        AggregationFunctions::aggregate(self.aggregation_id, vals)
    }


    fn distance(&self, other: &StandardNode, weight_coeff: f64) -> f64 {
        let mut tmp = 
            (self.bias - other.bias).abs() + (self.weight - other.weight).abs();

        if self.activation_id != other.activation_id {
            tmp += 1.0;
        }

        if self.aggregation_id != other.aggregation_id {
            tmp += 1.0;
        }

        tmp * weight_coeff
    }
}