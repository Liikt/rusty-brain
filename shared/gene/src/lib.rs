#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeType {
    Input,

    Output,

    Hidden,

    Bias
}

pub trait NodeGene {
    fn mutate_function(&mut self);
    fn get_key(&self) -> u64;
    fn get_type(&self) -> NodeType;
    fn activate(&self, val: f64) -> f64;
    fn aggregate(&self, vals: Vec<f64>) -> f64;
}
