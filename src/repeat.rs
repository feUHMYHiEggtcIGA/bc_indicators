use crate::prelude::*;

#[derive(Debug, PartialEq, PartialOrd, Clone, Default)]
pub struct REPEAT {
    pub value: f64,
}

impl REPEAT {
    pub fn new(value: f64) -> Self {
        Self { value }
    }
}

impl Indicator for REPEAT {
    fn w(&self) -> usize {
        0
    }
    fn ind(&self, _: &[f64]) -> f64 {
        self.value
    }
    fn init_bf(&self, _in_: &[Vec<f64>]) {}
    fn execute_bf(&self) {}
    fn ind_f(&self, _: &[Vec<f64>]) -> f64 {
        self.value
    }
    fn ind_vec(&self, in_: &[Vec<f64>]) -> Vec<f64> {
        (0..in_.len()).map(|_| self.value).collect()
    }
}

impl IndicatorExt for REPEAT {
    fn ind_coll<C>(&self, in_: &[Vec<f64>]) -> C
    where
        C: FromIterator<f64>,
    {
        (0..in_.len()).map(|_| self.value).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::LazyLock;

    use crate::prelude_tests::prelude::*;

    static RES: f64 = 1.0;
    static IN_: LazyLock<Vec<Vec<f64>>> = LazyLock::new(|| {
        (0..OPEN.len())
            .map(|i| vec![OPEN[i], CLOSE[i]])
            .collect::<Vec<Vec<f64>>>()
    });
    static SETTINGS_: LazyLock<REPEAT> = LazyLock::new(|| REPEAT::new(1.0));

    #[test]
    fn repeat_bf_res_1() {
        test_ind_bf_res_1((*SETTINGS_).clone(), &IN_, RES);
    }

    #[test]
    fn repeat_f_res_1() {
        test_f_res_1((*SETTINGS_).clone(), &IN_, RES);
    }

    #[test]
    fn repeat_coll_res_1() {
        test_coll_res_1((*SETTINGS_).clone(), &IN_, RES, 21);
    }

    #[test]
    fn repeat_coll_res_2() {
        test_coll_res_2((*SETTINGS_).clone(), &IN_, 30);
    }
}
