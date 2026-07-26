mod prelude;
use prelude::*;

use bc_indicators::rma::RMA;

static IN_: LazyLock<Vec<Vec<f64>>> = LazyLock::new(|| {
    OPEN.iter()
        .copied()
        .map(|v| vec![v])
        .collect::<Vec<Vec<f64>>>()
});

fn rma_bf_1(c: &mut Criterion) {
    let ind = RMA::new(4);
    ind.init_bf(&IN_);
    c.bench_function("rma_bf_1", |b| b.iter(|| ind.ind(&[OPEN_LAST])));
}

fn rma_f_1(c: &mut Criterion) {
    let ind = RMA::new(4);
    c.bench_function("rma_f_1", |b| b.iter(|| ind.ind_f(&IN_)));
}

fn rma_coll_1(c: &mut Criterion) {
    let ind = RMA::new(4);
    c.bench_function("rma_coll_1", |b| b.iter(|| ind.ind_coll::<Vec<f64>>(&IN_)));
}

criterion_group!(benches, rma_bf_1, rma_f_1, rma_coll_1);
criterion_main!(benches);
