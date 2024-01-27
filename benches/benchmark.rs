use criterion::{criterion_group, criterion_main, Criterion};
// use euler::get_solution;

pub fn criterion_benchmark(_c: &mut Criterion) {
    // just a stub
    // let solution = get_solution(1).unwrap();
    // c.bench_function("p001", |b| b.iter(solution));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
