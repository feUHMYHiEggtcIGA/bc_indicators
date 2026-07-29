mod prelude;
use prelude::*;

use bc_indicators::mm_scaller::MM_SCALLER;

static IN_: LazyLock<Vec<Vec<f64>>> = LazyLock::new(|| {
    OPEN.iter()
        .copied()
        .map(|v| vec![v])
        .collect::<Vec<Vec<f64>>>()
});

fn mm_scaller_bf_1(c: &mut Criterion) {
    let ind = MM_SCALLER::new(20);
    ind.init_bf(&IN_);
    c.bench_function("mm_scaller_bf_1", |b| b.iter(|| ind.ind(&[OPEN_LAST])));
}

criterion_group!(benches, mm_scaller_bf_1,);
criterion_main!(benches);
