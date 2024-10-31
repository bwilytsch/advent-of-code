use std::fs;

use advent_of_code::day_1::{part_one, part_two};
use criterion::{criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    let input = fs::read_to_string("./inputs/2019/001/part1.txt");

    match input {
        Ok(input) => {
            c.bench_function("2019 | day 1 | part 1", |b| b.iter(|| part_one(&input)));
            c.bench_function("2019 | day 1 | part 2", |b| b.iter(|| part_two(&input)));
        }
        Err(e) => {
            println!("error: {}", e);
        }
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
