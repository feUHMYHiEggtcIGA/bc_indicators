mod prelude;
use prelude::*;

use bc_indicators::minus::MINUS;

static IN_: LazyLock<Vec<Vec<f64>>> = LazyLock::new(|| {
    (0..OPEN.len())
        .map(|i| vec![OPEN[i], CLOSE[i]])
        .collect::<Vec<Vec<f64>>>()
});

fn minus_bf_1(c: &mut Criterion) {
    let ind = MINUS;
    ind.init_bf(&IN_);
    c.bench_function("minus_bf_1", |b| {
        b.iter(|| ind.ind(&[OPEN_LAST, CLOSE_LAST]))
    });
}

fn minus_f_1(c: &mut Criterion) {
    let ind = MINUS;
    c.bench_function("minus_f_1", |b| b.iter(|| ind.ind_f(&IN_)));
}

fn minus_coll_1(c: &mut Criterion) {
    let ind = MINUS;
    c.bench_function("minus_coll_1", |b| {
        b.iter(|| ind.ind_coll::<Vec<f64>>(&IN_))
    });
}

criterion_group!(benches, minus_bf_1, minus_f_1, minus_coll_1);
criterion_main!(benches);
