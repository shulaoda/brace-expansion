use brace_expansion::Pattern;
use criterion::{criterion_group, criterion_main, Criterion};

fn bench(c: &mut Criterion) {
  let glob = b"some/{a,b{c,d}f,e}/ccc.{png,jpg}";
  c.bench_function("brace_expansion", |b| {
    b.iter(|| {
      let mut node = Pattern::with(glob).unwrap();
      while node.trigger(glob, node.value.len()) {}
    })
  });
}

criterion_group!(benches, bench);
criterion_main!(benches);
