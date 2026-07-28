use bc_utils_lg::traits::w::W;

use crate::prelude::*;

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct AVG;

impl W for AVG {
    fn w(&self) -> usize {
        0
    }
}
impl Indicator for AVG {
    fn init_bf(&self, _in_: &[Vec<f64>]) {}
    fn execute_bf(&self) {}
    fn ind(&self, in_: &[f64]) -> f64 {
        in_.into_iter().sum::<f64>() / in_.len() as f64
    }
    fn ind_f(&self, in_: &[Vec<f64>]) -> f64 {
        self.ind(in_.last().expect("no elements in slice"))
    }
    fn ind_vec(&self, in_: &[Vec<f64>]) -> Vec<f64> {
        in_.iter().map(|x| self.ind(x)).collect()
    }
}

impl IndicatorExt for AVG {
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
    use std::sync::LazyLock;

    use crate::prelude_tests::prelude::*;

    static RES: f64 =
        (OPEN[OPEN.len() - 1] + CLOSE[OPEN.len() - 1] + HIGH[OPEN.len() - 1] + LOW[OPEN.len() - 1])
            / 4.0;
    static IN_: LazyLock<Vec<Vec<f64>>> = LazyLock::new(|| {
        (0..OPEN.len())
            .map(|i| vec![OPEN[i], CLOSE[i], HIGH[i], LOW[i]])
            .collect::<Vec<Vec<f64>>>()
    });

    #[test]
    fn avg_bf_res_1() {
        test_ind_bf_res_1(AVG, &IN_, RES);
    }

    #[test]
    fn avg_f_res_1() {
        test_f_res_1(AVG, &IN_, RES);
    }

    #[test]
    fn avg_coll_res_1() {
        test_coll_res_1(AVG, &IN_, RES, 21);
    }

    #[test]
    fn avg_coll_res_2() {
        test_coll_res_2(AVG, &IN_, 30);
    }
}
