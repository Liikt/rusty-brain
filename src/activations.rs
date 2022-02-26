//! https://github.com/CodeReclaimers/neat-python/blob/c2b79c88667a1798bfe33c00dd8e251ef8be41fa/neat/activations.py
pub struct ActivationFunctions;

impl ActivationFunctions {
    fn sigmoid(val: f64) -> f64 {
        let tmp = -60.0_f64.max(60.0_f64.min(5.0 * val));
        1.0 / (1.0 + (-tmp).exp())
    }

    fn tanh(val: f64) -> f64 {
        -60.0_f64.max(60.0_f64.min(2.5 * val)).tanh()
    }

    fn sin(val: f64) -> f64 {
        -60.0_f64.max(60.0_f64.min(5.0 * val)).sin()
    }

    fn gauss(val: f64) -> f64 {
        let tmp = -3.4_f64.max(3.4_f64.min(val));
        (-5.0 * tmp * tmp).exp()
    }

    fn relu(val: f64) -> f64 {
        val.max(0.0)
    }

    fn elu(val: f64) -> f64 {
        if val > 0.0 { val } else { val.exp_m1() }
    }

    fn lelu(val: f64) -> f64 {
        if val > 0.0 { val } else { val * 0.005 }
    }

    fn selu(val: f64) -> f64 {
        let lam = 1.0507009873554805;
        let alpha = 1.6732632423543772;
        if val > 0.0 { lam * val } else { lam * alpha * (val.exp_m1()) }
    }

    fn softplus(val: f64) -> f64 {
        let tmp = -60.0_f64.max(60.0_f64.min(5.0 * val));
        0.2 * tmp.exp().ln_1p()
    }

    fn identity(val: f64) -> f64 {
        val
    }

    fn clamped(val: f64) -> f64 {
        val.clamp(-1.0, 1.0)
    }

    fn inv(val: f64) -> f64 {
        let tmp = 1.0 / val;
        if tmp.is_infinite() { val } else { tmp }
    }

    fn log(val: f64) -> f64 {
        val.max(1.0_f64.powi(-7)).ln()
    }

    fn exp(val: f64) -> f64 {
        -60.0_f64.max(60.0_f64.min(5.0 * val)).exp()
    }

    fn abs(val: f64) -> f64 {
        val.abs()
    }

    fn hat(val: f64) -> f64 {
        0.0_f64.max(1.0 - val)
    }

    fn square(val: f64) -> f64 {
        val * val
    }

    fn cube(val: f64) -> f64 {
        val * val * val
    }

    pub fn get_num_funcs() -> u8 {
        18
    }

    pub fn activate(fid: u8, val: f64) -> f64 {
        match fid {
            0  => Self::sigmoid(val),
            1  => Self::tanh(val),
            2  => Self::sin(val),
            3  => Self::gauss(val),
            4  => Self::relu(val),
            5  => Self::elu(val),
            6  => Self::lelu(val),
            7  => Self::selu(val),
            8  => Self::softplus(val),
            9  => Self::identity(val),
            10 => Self::clamped(val),
            11 => Self::inv(val),
            12 => Self::log(val),
            13 => Self::exp(val),
            14 => Self::abs(val),
            15 => Self::hat(val),
            16 => Self::square(val),
            17 => Self::cube(val),

            _ => panic!("Max Function ID: {} Got: {}",
                Self::get_num_funcs(), fid)
        }
    }
}