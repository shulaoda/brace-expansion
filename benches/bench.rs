use brace_expansion::Pattern;
use criterion::{criterion_group, criterion_main, Criterion};

const GLOB: &[u8] = b"some/{a,b\\[[ab{c,d}]{c,d}f,e}/ccc.{png,jpg}";

fn bench(c: &mut Criterion) {
  c.bench_function("bench", |b| {
    b.iter(|| {
      if let Some(branch) = Some(vec![(0, 3), (0, 2), (0, 2)]) {
        if branch.is_empty() {
          let value = GLOB.to_vec();
          let shadow = Vec::new();

          Pattern {
            value,
            branch,
            shadow,
          };
        } else {
          let value = Vec::with_capacity(GLOB.len());
          let shadow = Vec::with_capacity(branch.len());

          let mut pattern = Pattern {
            value,
            branch,
            shadow,
          };
        }
      }
    })
  });
}

fn brace_new(c: &mut Criterion) {
  c.bench_function("brace_new", |b| {
    b.iter(|| {
      Pattern::new(GLOB).unwrap();
    })
  });
}

fn brace_parse(c: &mut Criterion) {
  c.bench_function("brace_parse", |b| {
    b.iter(|| {
      Pattern::parse(GLOB).unwrap();
    })
  });
}

fn brace_track(c: &mut Criterion) {
  c.bench_function("brace_track", |b| {
    let mut node = Pattern::new(GLOB).unwrap();
    b.iter(|| {
      node.track(GLOB);
    })
  });
}

fn brace_trigger(c: &mut Criterion) {
  c.bench_function("brace_trigger", |b| {
    let mut node = Pattern::new(GLOB).unwrap();
    b.iter(|| {
      node.trigger(GLOB, node.value.len());
    })
  });
}

fn brace_expansion(c: &mut Criterion) {
  c.bench_function("brace_expansion", |b| {
    b.iter(|| {
      let mut node = Pattern::new(GLOB).unwrap();
      while node.trigger(GLOB, node.value.len()) {}
    })
  });
}

criterion_group!(
  benches,
  bench,
  brace_new,
  brace_parse,
  brace_track,
  brace_trigger,
  brace_expansion
);
criterion_main!(benches);
