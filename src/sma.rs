use bc_utils::nums::avg;

use crate::prelude::*;

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct SmaParams {
    pub window: usize,
}

impl Default for SmaParams {
    fn default() -> Self {
        Self { window: 14 }
    }
}

impl SmaParams {
    pub fn new(window: usize) -> Self {
        Self {
            window,
            ..Default::default()
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Default, Clone)]
pub struct SmaBf {
    src_l: Vec<f64>,
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct SMA {
    pub params: SmaParams,
    bf: RefCell<SmaBf>,
    bf_state: RefCell<SmaBf>,
}

impl Default for SMA {
    fn default() -> Self {
        Self {
            params: Default::default(),
            bf: Default::default(),
            bf_state: Default::default(),
        }
    }
}

impl SMA {
    pub fn new(window: usize) -> Self {
        Self {
            params: SmaParams::new(window),
            ..Default::default()
        }
    }
}

impl W for SMA {
    fn w(&self) -> usize {
        self.params.window
    }
}

impl Indicator for SMA {
    fn init_bf(&self, in_: &[Vec<f64>]) {
        self.bf.borrow_mut().src_l = in_[in_.len() - self.w()..]
            .iter()
            .map(|v| v[0])
            .collect::<Vec<f64>>();
        self.bf_state.borrow_mut().src_l = self.bf.borrow().src_l.clone();
    }
    fn execute_bf(&self) {
        *self.bf.borrow_mut() = self.bf_state.borrow().clone();
    }
    fn ind(&self, in_: &[f64]) -> f64 {
        self.bf_state.borrow_mut().src_l = {
            let mut bind = self.bf.borrow().src_l[1..].to_vec();
            bind.push(in_[0]);
            bind
        };
        avg(&self.bf_state.borrow().src_l)
    }
}

impl IndicatorExt for SMA {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::LazyLock;

    use crate::prelude_tests::prelude::*;

    static IN_: LazyLock<Vec<Vec<f64>>> = LazyLock::new(|| {
        OPEN.iter()
            .copied()
            .map(|v| vec![v])
            .collect::<Vec<Vec<f64>>>()
    });
    static RES: LazyLock<f64> =
        LazyLock::new(|| OPEN[OPEN.len() - 10..].into_iter().sum::<f64>() / 10.0);

    #[test]
    fn sma_bf_res_1() {
        let settings = SMA::new(10);
        test_ind_bf_res_1(settings, &IN_, *RES);
    }

    #[test]
    fn sma_coll_res_1() {
        let settings = SMA::new(10);
        test_coll_res_1(settings, &IN_, 10);
    }
}
