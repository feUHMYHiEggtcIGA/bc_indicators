use crate::prelude::*;

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct REM;

impl Indicator for REM {
    fn w(&self) -> usize {
        0
    }
    fn ind(&self, math_operations: &[f64]) -> f64 {
        math_operations[0] % math_operations[1]
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

impl IndicatorExt for REM {
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

    static RES: f64 = OPEN[OPEN.len() - 1] % CLOSE[OPEN.len() - 1];
    static IN_: LazyLock<Vec<Vec<f64>>> = LazyLock::new(|| {
        (0..OPEN.len())
            .map(|i| vec![OPEN[i], CLOSE[i]])
            .collect::<Vec<Vec<f64>>>()
    });

    #[test]
    fn rem_bf_res_1() {
        test_ind_bf_res_1(REM, &IN_, RES);
    }

    #[test]
    fn rem_f_res_1() {
        test_f_res_1(REM, &IN_, RES);
    }

    #[test]
    fn rem_coll_res_1() {
        test_coll_res_1(REM, &IN_, RES, 21);
    }

    #[test]
    fn rem_coll_res_2() {
        test_coll_res_2(REM, &IN_, 30);
    }
}
