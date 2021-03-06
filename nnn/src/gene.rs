use std::rc::Rc;
use std::cell::RefCell;

use rand::random;

use config::Config;
use gene::{NodeType, NodeGene, ConnectionGene};

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

#[derive(Debug, Clone, Copy)]
pub struct StandardConnection {
    src: u64,
    dst: u64,
    weight: f64,
    enabled: bool,
    innovation: u64
}

impl StandardConnection {
    pub fn new(src: u64, dst: u64, innovation: u64) -> Self {
        StandardConnection { src, dst, weight: 1.0, enabled: true, innovation }
    }
}

impl ConnectionGene for StandardConnection {
    fn get_src(&self) -> u64 {
        self.src
    }

    fn get_dst(&self) -> u64 {
        self.dst
    }

    fn get_weight(&self) -> f64 {
        self.weight
    }

    fn get_innovation(&self) -> u64 {
        self.innovation
    }

    fn is_enabled(&self) -> bool {
        self.enabled
    }

    fn distance(&self, other: &Self, weight_coeff: f64) -> f64 {
        let mut tmp = (self.weight - other.weight).abs();
        if self.enabled != other.enabled {
            tmp += 1.0
        }
        tmp * weight_coeff
    }
}