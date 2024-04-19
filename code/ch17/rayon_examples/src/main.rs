#![allow(unused_imports)]
use rayon::prelude::*;

pub fn ex01() {
    let mut my_vec = vec![0; 2_000_000];
    my_vec
        .iter_mut()
        .enumerate()
        .for_each(|(index, number)| *number += index + 1);
    println!("{:?}", &my_vec[5000..5005]);
}

pub fn ex02() {
    let mut my_vec = vec![0; 2_000_000];
    my_vec
        .par_iter_mut()
        .enumerate()
        .for_each(|(index, number)| *number += index + 1);
}

pub fn ex03() {
    use std::thread::available_parallelism;

    println!(
        "Estimated parallelism on this computer: {:?}",
        available_parallelism()
    );
    let mut without_rayon = vec![];
    let mut with_rayon = vec![];
    for _ in 0..10 {
        let mut my_vec = vec![0; 2_000_000];
        let now = std::time::Instant::now();
        my_vec.iter_mut().enumerate().for_each(|(index, number)| {
            *number += index + 1;
            *number -= index + 1;
        });
        let elapsed = now.elapsed();
        without_rayon.push(elapsed.as_micros());
        let mut my_vec = vec![0; 2_000_000];
        let now = std::time::Instant::now();
        my_vec
            .par_iter_mut()
            .enumerate()
            .for_each(|(index, number)| {
                *number += index + 1;
                *number -= index + 1;
            });
        let elapsed = now.elapsed();
        with_rayon.push(elapsed.as_micros());
    }
    println!(
        "Average time without rayon: {} microseconds",
        without_rayon.into_iter().sum::<u128>() / 10
    );
    println!(
        "Average time with rayon: {} microseconds",
        with_rayon.into_iter().sum::<u128>() / 10
    );
}

fn main() {
    ex03();
}
