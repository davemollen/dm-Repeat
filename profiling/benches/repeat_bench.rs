#[path = "../src/utils.rs"]
mod utils;
use criterion::{criterion_group, criterion_main, Criterion};
use repeat::Repeat;
use utils::generate_signal_stream;

fn repeat_bench(c: &mut Criterion) {
  let mut repeat = Repeat::new(44100.);
  let signal_stream = generate_signal_stream(44100);

  c.bench_function("repeat", |b| {
    b.iter(|| {
      for signal in &signal_stream {
        repeat.process(*signal, 100., 16, 1., -0.25, true);
      }
    })
  });
}

criterion_group!(benches, repeat_bench);
criterion_main!(benches);
