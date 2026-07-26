#![allow(non_camel_case_types)]

use std::cell::RefCell;

use bc_utils_lg::types::maps::MAP;

use std::any::Any;

fn ind_coll<C, T>(indicator: &T, in_: &[Vec<f64>]) -> C
where
    C: FromIterator<f64>,
    T: Indicator,
    T: ?Sized,
{
    if indicator.w() != 0 {
        indicator.init_bf(&in_[..indicator.w() - 1]);
    } else {
        indicator.init_bf(in_);
    }
    in_.iter()
        .enumerate()
        .map(|v| {
            if v.0 < indicator.w().checked_sub(1).unwrap_or(indicator.w()) {
                f64::NAN
            } else {
                let res = indicator.ind(v.1);
                indicator.execute_bf();
                res
            }
        })
        .collect()
}

pub trait Indicator: Any {
    fn w(&self) -> usize;
    fn init_bf(&self, in_: &[Vec<f64>]);
    fn execute_bf(&self);
    fn ind(&self, in_: &[f64]) -> f64;
    fn ind_f(&self, in_: &[Vec<f64>]) -> f64 {
        if self.w() != 0 {
            self.init_bf(&in_[in_.len() - self.w()..in_.len() - 1]);
        } else {
            self.init_bf(in_);
        }
        self.ind(&in_[in_.len() - 1])
    }
    fn ind_vec(&self, in_: &[Vec<f64>]) -> Vec<f64> {
        ind_coll(self, in_)
    }
}

pub trait IndicatorExt: Indicator {
    fn ind_coll<C>(&self, in_: &[Vec<f64>]) -> C
    where
        C: FromIterator<f64>,
    {
        ind_coll(self, in_)
    }
}
