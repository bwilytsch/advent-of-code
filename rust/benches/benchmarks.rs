use std::fs;

use advent_of_code::year_2024;
use criterion::{criterion_group, criterion_main, Criterion};

fn day_1(c: &mut Criterion) {
    let input = fs::read_to_string("./inputs/2024/001/input.txt");

    match input {
        Ok(input) => {
            c.bench_function("2024 | day 1 | part 1", |b| {
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

fn day_3(c: &mut Criterion) {
    let input = fs::read_to_string("./inputs/2024/003/input.txt");

    match input {
        Ok(input) => {
            c.bench_function("2024 | day 3 | part 1", |b| {
                b.iter(|| year_2024::day_03::part_one(&input))
            });
            c.bench_function("2024 | day 3 | part 2", |b| {
                b.iter(|| year_2024::day_03::part_two(&input))
            });
        }
        Err(e) => {
            println!("error: {}", e);
        }
    }
}

fn day_4(c: &mut Criterion) {
    let input = fs::read_to_string("./inputs/2024/004/input.txt");

    match input {
        Ok(input) => {
            c.bench_function("2024 | day 4 | part 1", |b| {
                b.iter(|| year_2024::day_04::part_one(&input))
            });
            c.bench_function("2024 | day 4 | part 2", |b| {
                b.iter(|| year_2024::day_04::part_two(&input))
            });
        }
        Err(e) => {
            println!("error: {}", e);
        }
    }
}

criterion_group!(benches, day_1, day_2, day_3, day_4);
criterion_main!(benches);
