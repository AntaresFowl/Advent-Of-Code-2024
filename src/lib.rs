#![feature(portable_simd)]
#![feature(iter_map_windows)]

use aoc_runner_derive::aoc_lib;

#[global_allocator]
static ALLOCATOR: mimalloc::MiMalloc = mimalloc::MiMalloc;

pub mod day1;
pub mod day2;

aoc_lib! { year = 2024 }
