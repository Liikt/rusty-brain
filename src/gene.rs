use crate::Config;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeType {
    Input,

    Output,

    Hidden,

    Bias
}

pub trait NodeGene {
    fn mutate_value(&mut self);
    fn get_innovation(&self) -> u64;
    fn get_type(&self) -> NodeType;
}