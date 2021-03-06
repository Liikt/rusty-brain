#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeType {
    Input,

    Output,

    Hidden,

    Bias
}

pub trait NodeGene: std::clone::Clone {
    fn get_key(&self) -> u64;
    fn get_type(&self) -> NodeType;
    fn get_bias(&self) -> f64;
    fn get_weight(&self) -> f64;
    fn activate(&self, val: f64) -> f64;
    fn aggregate(&self, vals: Vec<f64>) -> f64;
    fn distance(&self, other: &Self, weight_coeff: f64) -> f64;
}

pub trait ConnectionGene {
    fn get_src(&self) -> u64;
    fn get_dst(&self) -> u64;
    fn get_weight(&self) -> f64;
    fn get_innovation(&self) -> u64;
    fn is_enabled(&self) -> bool;
    fn distance(&self, other: &Self, weight_coeff: f64) -> f64;
}