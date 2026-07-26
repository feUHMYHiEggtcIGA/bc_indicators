mod prelude;
use bc_indicators::percent::PERCENT;
use prelude::*;

static IN_: LazyLock<Vec<Vec<f64>>> = LazyLock::new(|| {
    (0..OPEN.len())
        .map(|i| vec![OPEN[i], CLOSE[i]])
        .collect::<Vec<Vec<f64>>>()
});

fn percent_bf_1(c: &mut Criterion) {
    let ind = PERCENT;
    ind.init_bf(&IN_);
    c.bench_function("percent_bf_1", |b| {
        b.iter(|| ind.ind(&[OPEN_LAST, CLOSE_LAST]))
    });
}

fn percent_f_1(c: &mut Criterion) {
    let ind = PERCENT;
    c.bench_function("percent_f_1", |b| b.iter(|| ind.ind_f(&IN_)));
}

fn percent_coll_1(c: &mut Criterion) {
    let ind = PERCENT;
    c.bench_function("percent_coll_1", |b| {
        b.iter(|| ind.ind_coll::<Vec<f64>>(&IN_))
    });
}

criterion_group!(benches, percent_bf_1, percent_f_1, percent_coll_1);
criterion_main!(benches);
