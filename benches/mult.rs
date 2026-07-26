mod prelude;
use prelude::*;

use bc_indicators::mult::MULT;

static IN_: LazyLock<Vec<Vec<f64>>> = LazyLock::new(|| {
    (0..OPEN.len())
        .map(|i| vec![OPEN[i], CLOSE[i]])
        .collect::<Vec<Vec<f64>>>()
});

fn mult_bf_1(c: &mut Criterion) {
    let ind = MULT;
    ind.init_bf(&IN_);
    c.bench_function("mult_bf_1", |b| {
        b.iter(|| ind.ind(&[OPEN_LAST, CLOSE_LAST]))
    });
}

fn mult_f_1(c: &mut Criterion) {
    let ind = MULT;
    c.bench_function("mult_f_1", |b| b.iter(|| ind.ind_f(&IN_)));
}

fn mult_coll_1(c: &mut Criterion) {
    let ind = MULT;
    c.bench_function("mult_coll_1", |b| b.iter(|| ind.ind_coll::<Vec<f64>>(&IN_)));
}

criterion_group!(benches, mult_bf_1, mult_f_1, mult_coll_1);
criterion_main!(benches);
