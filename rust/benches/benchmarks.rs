use std::fs;

use advent_of_code::year_2024;
use criterion::{criterion_group, criterion_main, Criterion};

fn day_1(c: &mut Criterion) {
    let input = fs::read_to_string("./inputs/2024/001/input.txt");

    match input {
        Ok(input) => {
            c.bench_function("2019 | day 1 | part 1", |b| {
                b.iter(|| year_2024::day_01::part_one(&input))
            });
            c.bench_function("2024 | day 1 | part 2", |b| {
                b.iter(|| year_2024::day_01::part_two(&input))
            });
        }
        Err(e) => {
            println!("error: {}", e);
        }
    }
}

fn day_2(c: &mut Criterion) {
    let input = fs::read_to_string("./inputs/2024/002/input.txt");

    match input {
        Ok(input) => {
            c.bench_function("2024 | day 2 | part 1", |b| {
                b.iter(|| year_2024::day_02::part_one(&input))
            });
            c.bench_function("2024 | day 2 | part 2", |b| {
                b.iter(|| year_2024::day_02::part_two(&input))
            });
        }
        Err(e) => {
            println!("error: {}", e);
        }
    }
}

criterion_group!(benches, day_1, day_2);
criterion_main!(benches);
