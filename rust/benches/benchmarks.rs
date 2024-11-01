use std::fs;

use advent_of_code::day_1;
use advent_of_code::day_2;
use advent_of_code::day_3;
use criterion::{criterion_group, criterion_main, Criterion};

fn day_1(c: &mut Criterion) {
    let input = fs::read_to_string("./inputs/2019/001/part1.txt");

    match input {
        Ok(input) => {
            c.bench_function("2019 | day 1 | part 1", |b| {
                b.iter(|| day_1::part_one(&input))
            });
            c.bench_function("2019 | day 1 | part 2", |b| {
                b.iter(|| day_1::part_two(&input))
            });
        }
        Err(e) => {
            println!("error: {}", e);
        }
    }
}

fn day_2(c: &mut Criterion) {
    let input = fs::read_to_string("./inputs/2019/002/part1.txt");

    match input {
        Ok(input) => {
            c.bench_function("2019 | day 2 | part 1", |b| {
                b.iter(|| day_2::part_one(&input))
            });
            c.bench_function("2019 | day 2 | part 2", |b| {
                b.iter(|| day_2::part_two(&input, "19690720"))
            });
        }
        Err(e) => {
            println!("error: {}", e);
        }
    }
}

fn day_3(c: &mut Criterion) {
    let input = fs::read_to_string("./inputs/2019/003/input.txt");

    match input {
        Ok(input) => {
            c.bench_function("2019 | day 3 | part 1", |b| {
                b.iter(|| day_3::part_one(&input))
            });
            c.bench_function("2019 | day 3 | part 2", |b| {
                b.iter(|| day_3::part_two(&input))
            });
        }
        Err(e) => {
            println!("error: {}", e);
        }
    }
}

criterion_group!(benches, day_1, day_2, day_3);
criterion_main!(benches);
