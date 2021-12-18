use criterion::{black_box, criterion_group, criterion_main, Criterion};

use hashman_rs::HashEnv;

pub fn criterion_benchmark(c: &mut Criterion) {
  let env = HashEnv::setup();
  let mut sys = env.sys;
  c.bench_function("get pids", |b| {
    b.iter(|| sys.fetch_pids(black_box(&env.hash_path)))
  });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
