use brace_expansion::parse_brace;
use criterion::{criterion_group, criterion_main, Criterion};

fn brace(b: &mut Criterion) {
  b.bench_function("brace", |b| {
    b.iter(|| parse_brace("some/**/{a,b{c,de},{g,l}}.js".as_bytes()))
  });
}

fn more_brace(b: &mut Criterion) {
  b.bench_function("more_brace", |b| {
    b.iter(|| parse_brace("{a,b{c,de},{g,l}}{c,d}{e}{f}{g}{h}{i}{j}{k}{l}.js".as_bytes()))
  });
}

fn empty_brace(b: &mut Criterion) {
  b.bench_function("empty_brace", |b| {
    b.iter(|| parse_brace("{a,b{c,de},{g,l}}{}{}{}{}{}{}{}{}{}.js".as_bytes()))
  });
}

fn without_brace(b: &mut Criterion) {
  b.bench_function("without_brace", |b| {
    b.iter(|| parse_brace("some/**/[e,bcc,de*,[g,l]].js".as_bytes()))
  });
}

criterion_group!(benches, brace, more_brace, empty_brace, without_brace);
criterion_main!(benches);
