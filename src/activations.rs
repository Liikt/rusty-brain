//! https://github.com/CodeReclaimers/neat-python/blob/c2b79c88667a1798bfe33c00dd8e251ef8be41fa/neat/activations.py
pub struct ActivationFunctions;

impl ActivationFunctions {
    pub fn sigmoid(val: f64) -> f64 {
        let tmp = -60.0_f64.max(60.0_f64.min(5.0 * val));
        1.0 / (1.0 + (-tmp).exp())
    }

    pub fn tanh(val: f64) -> f64 {
        -60.0_f64.max(60.0_f64.min(2.5 * val)).tanh()
    }

    pub fn sin(val: f64) -> f64 {
        -60.0_f64.max(60.0_f64.min(5.0 * val)).sin()
    }

    pub fn gauss(val: f64) -> f64 {
        let tmp = -3.4_f64.max(3.4_f64.min(val));
        (-5.0 * tmp * tmp).exp()
    }

    pub fn relu(val: f64) -> f64 {
        val.max(0.0)
    }

    pub fn elu(val: f64) -> f64 {
        if val > 0.0 { val } else { val.exp_m1() }
    }

    pub fn lelu(val: f64) -> f64 {
        if val > 0.0 { val } else { val * 0.005 }
    }

    pub fn selu(val: f64) -> f64 {
        let lam = 1.0507009873554805;
        let alpha = 1.6732632423543772;
        if val > 0.0 { lam * val } else { lam * alpha * (val.exp_m1()) }
    }

    pub fn softplus(val: f64) -> f64 {
        let tmp = -60.0_f64.max(60.0_f64.min(5.0 * val));
        0.2 * tmp.exp().ln_1p()
    }

    pub fn identity(val: f64) -> f64 {
        val
    }

    pub fn clamped(val: f64) -> f64 {
        val.clamp(-1.0, 1.0)
    }

    pub fn inv(val: f64) -> f64 {
        let tmp = 1.0 / val;
        if tmp.is_infinite() { val } else { tmp }
    }

    pub fn log(val: f64) -> f64 {
        val.max(1.0_f64.powi(-7)).ln()
    }

    pub fn exp(val: f64) -> f64 {
        -60.0_f64.max(60.0_f64.min(5.0 * val)).exp()
    }

    pub fn abs(val: f64) -> f64 {
        val.abs()
    }

    pub fn hat(val: f64) -> f64 {
        0.0_f64.max(1.0 - val)
    }

    pub fn square(val: f64) -> f64 {
        val * val
    }

    pub fn cube(val: f64) -> f64 {
        val * val * val
    }
}