#![allow(non_camel_case_types)]
use crate::prelude::*;

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct OSC_MULT {
    pub th_short: f64,
    pub th_long: f64,
    pub max_value: f64,
}

impl Default for OSC_MULT {
    fn default() -> Self {
        Self {
            th_short: 0.15,
            th_long: 0.15,
            max_value: 1.,
        }
    }
}

impl OSC_MULT {
    pub fn new(th_short: f64, th_long: f64, max_value: f64) -> Self {
        Self {
            th_short,
            th_long,
            max_value,
        }
    }
}

impl W for OSC_MULT {
    fn w(&self) -> usize {
        0
    }
}

impl Indicator for OSC_MULT {
    fn ind(&self, in_: &[f64]) -> f64 {
        let diff: f64;
        let v2: f64;
        let v_b = in_[0];

        if v_b >= (self.max_value - self.th_short) {
            diff = self.th_short;
            v2 = self.max_value - v_b;
        } else if v_b <= self.th_long {
            diff = self.th_long;
            v2 = v_b;
        } else {
            diff = v_b;
            v2 = v_b;
        }
        (diff - v2) / diff
    }
    fn init_bf(&self, _in_: &[Vec<f64>]) {}
    fn execute_bf(&self) {}
    fn ind_f(&self, in_: &[Vec<f64>]) -> f64 {
        self.ind(in_.last().expect("no elements in slice"))
    }
    fn ind_vec(&self, in_: &[Vec<f64>]) -> Vec<f64> {
        in_.iter().map(|x| self.ind(x)).collect()
    }
}

impl IndicatorExt for OSC_MULT {
    fn ind_coll<C>(&self, in_: &[Vec<f64>]) -> C
    where
        C: FromIterator<f64>,
    {
        in_.iter().map(|x| self.ind(x)).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude_tests::prelude::*;
    use std::sync::LazyLock;

    static RES: f64 = 0.5;
    static IN_: LazyLock<Vec<Vec<f64>>> = LazyLock::new(|| vec![vec![85.0]; 5]);

    #[test]
    fn osc_mult_bf_res_1() {
        let settings = OSC_MULT::new(30.0, 70.0, 100.0);
        test_ind_bf_res_1(settings, &IN_, RES);
    }

    #[test]
    fn osc_mult_f_res_1() {
        let settings = OSC_MULT::new(30.0, 70.0, 100.0);
        test_f_res_1(settings, &IN_, RES);
    }

    #[test]
    fn osc_mult_coll_res_1() {
        let settings = OSC_MULT::new(30.0, 70.0, 100.0);
        test_coll_res_1(settings, &IN_, RES, 2);
    }

    #[test]
    fn osc_mult_coll_res_2() {
        let settings = OSC_MULT::new(30.0, 70.0, 100.0);
        test_coll_res_2(settings, &IN_, 2);
    }
}
