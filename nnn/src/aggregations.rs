pub struct AggregationFunctions;

impl AggregationFunctions {
    fn product(vals: Vec<f64>) -> f64 {
        vals.iter().fold(1.0, |acc, x| acc * x)
    }

    fn sum(vals: Vec<f64>) -> f64 {
        vals.iter().sum()
    }

    fn max(vals: Vec<f64>) -> f64 {
        vals.into_iter().reduce(|acc, x| if acc < x { x } else { acc })
            .unwrap_or(0.0)
    }

    fn min(vals: Vec<f64>) -> f64 {
        vals.into_iter().reduce(|acc, x| if acc > x { x } else { acc })
            .unwrap_or(0.0)
    }

    fn max_abs(vals: Vec<f64>) -> f64 {
        vals.into_iter().reduce(|acc, x| {
            if acc < x.abs() { 
                x.abs() 
            } else { 
                acc 
            }
        }).unwrap_or(0.0)
    }

    fn min_abs(vals: Vec<f64>) -> f64 {
        vals.into_iter().reduce(|acc, x| {
            if acc > x.abs() { 
                x.abs() 
            } else { 
                acc 
            }
        }).unwrap_or(0.0)
    }

    pub fn get_num_funcs() -> u8 {
        6
    }

    pub fn aggregate(fid: u8, vals: Vec<f64>) -> f64 {
        match fid {
            0 => Self::product(vals),
            1 => Self::sum(vals),
            2 => Self::max(vals),
            3 => Self::min(vals),
            4 => Self::max_abs(vals),
            5 => Self::min_abs(vals),

            _ => panic!("Max Function ID: {} Got: {}", 
                Self::get_num_funcs(), fid)
        }
    }
}