use brace_expansion::*;
use criterion::{criterion_group, criterion_main, Criterion};

static PATH: &'static str = "some/abc/ccc.666";
static GLOB: &'static str = "some/abc/ccc.{{1,2,3,4,5,6},2,3,4,5,6}{1,{1,2,3,4,5,6},3,4,5,6}{1,2,{1,2,3,4,5,{1,2,3,4,5,6}},4,5,6}";

fn mine(b: &mut Criterion) {
  b.bench_function("mine", |b| {
    b.iter(|| {
      let glob = GLOB.as_bytes();
      let mut node = Pattern::with(glob).unwrap();

      while node.trigger(glob, node.value.len()) {}
    })
  });
}

criterion_group!(benches, mine);
criterion_main!(benches);
