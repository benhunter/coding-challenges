use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day_22::{solve_part1, solve_part2};

pub fn day_22(c: &mut Criterion) {
    let input = include_str!("../input.txt");
    let mut group = c.benchmark_group("day-22");
    //group.bench_function("part1", |b| b.iter(|| solve_part1(black_box(input))));
    group.bench_function("part2", |b| b.iter(|| solve_part2(black_box(input))));
    //group.bench_function("both", |b| b.iter(|| {
    //    solve_part1(black_box(input));
    //    solve_part2(black_box(input));
    //}));
}

criterion_group!(benches, day_22);
criterion_main!(benches);
