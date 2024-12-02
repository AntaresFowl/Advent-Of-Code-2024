#![feature(portable_simd)]

use aoc_runner_derive::aoc_lib;

#[global_allocator]
static ALLOCATOR: mimalloc::MiMalloc = mimalloc::MiMalloc;
pub mod day1;

aoc_lib! { year = 2024 }
