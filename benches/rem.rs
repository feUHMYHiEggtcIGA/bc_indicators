mod prelude;
use prelude::*;

use bc_indicators::rem::REM;

static IN_: LazyLock<Vec<Vec<f64>>> = LazyLock::new(|| {
    (0..OPEN.len())
        .map(|i| vec![OPEN[i], CLOSE[i]])
        .collect::<Vec<Vec<f64>>>()
});

fn rem_bf_1(c: &mut Criterion) {
    let ind = REM;
    ind.init_bf(&IN_);
    c.bench_function("rem_bf_1", |b| b.iter(|| ind.ind(&[OPEN_LAST, CLOSE_LAST])));
}

fn rem_f_1(c: &mut Criterion) {
    let ind = REM;
    c.bench_function("rem_f_1", |b| b.iter(|| ind.ind_f(&IN_)));
}

fn rem_coll_1(c: &mut Criterion) {
    let ind = REM;
    c.bench_function("rem_coll_1", |b| b.iter(|| ind.ind_coll::<Vec<f64>>(&IN_)));
}

criterion_group!(benches, rem_bf_1, rem_f_1, rem_coll_1);
criterion_main!(benches);
