use std::time::Duration;

use criterion::{criterion_group, criterion_main, Criterion, Throughput};
use unicode_truncate::UnicodeTruncateStr;

fn roughly_cut(str: &str, size: usize) -> &str {
    if size >= str.len() {
        return str;
    }
    let mut end = size;
    while !str.is_char_boundary(end) {
        end += 1;
    }
    &str[..end]
}

fn criterion_benchmark(criterion: &mut Criterion) {
    const KB: usize = 1024;
    const TEXT: &str = include_str!("data/zhufu.txt");

    for &size in &[KB, 2 * KB, 4 * KB, 8 * KB, 16 * KB, 28 * KB] {
        let mut group = criterion.benchmark_group(format!("zhu fu/{size}"));
        group
            .sample_size(1000)
            .measurement_time(Duration::from_secs(20))
            .throughput(Throughput::Bytes(size as u64));
        let input = roughly_cut(TEXT, size);
        let max_width = input.len() / 2;
        group.bench_with_input("end", input, |bench, str| {
            bench.iter(|| str.unicode_truncate(max_width));
        });
        group.bench_with_input("start", input, |bench, str| {
            bench.iter(|| str.unicode_truncate_start(max_width));
        });
        group.bench_with_input("centered", input, |bench, str| {
            bench.iter(|| str.unicode_truncate_centered(max_width));
        });
        group.finish();
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
