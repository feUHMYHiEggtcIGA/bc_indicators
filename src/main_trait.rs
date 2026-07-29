#![allow(non_camel_case_types)]

use std::any::Any;

use bc_utils_lg::traits::w::W;
use dyn_clone::DynClone;

fn ind_coll<C, T>(indicator: &T, in_: &[Vec<f64>]) -> C
where
    C: FromIterator<f64>,
    T: Indicator,
    T: ?Sized,
{
    in_.iter()
        .enumerate()
        .map(|v| {
            let res = indicator.ind(v.1);
            indicator.execute_bf();
            res
        })
        .collect()
}

pub trait Indicator: Any + W + DynClone {
    fn init_bf(&self, in_: &[Vec<f64>]);
    fn execute_bf(&self);
    fn ind(&self, in_: &[f64]) -> f64;
    fn ind_vec(&self, in_: &[Vec<f64>]) -> Vec<f64> {
        ind_coll(self, in_)
    }
}

dyn_clone::clone_trait_object!(Indicator);

pub trait IndicatorExt: Indicator {
    fn ind_coll<C>(&self, in_: &[Vec<f64>]) -> C
    where
        C: FromIterator<f64>,
    {
        ind_coll(self, in_)
    }
}
