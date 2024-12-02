use std::simd::{num::*, Simd};

use aoc_runner_derive::aoc;
use nohash_hasher::{BuildNoHashHasher, IntMap};

#[aoc(day1, part1)]
#[inline]
pub fn part1(input: &str) -> i32 {
    let input = input.as_bytes();

    let mut first = Vec::with_capacity(input.len() / 3);
    let mut second = Vec::with_capacity(input.len() / 3);

    let mut num1 = 0;
    let mut num2 = 0;
    let mut current = true;
    for &byte in input {
        if byte == b'\n' || byte == b'\r' {
            first.push(num1);
            second.push(num2);

            num1 = 0;
            num2 = 0;
            current = true;

            continue;
        }
        if byte == b' ' {
            current = false;
            continue;
        }
        if current {
            num1 *= 10;
            num1 += (byte.wrapping_sub(48)) as i32;
        } else {
            num2 *= 10;
            num2 += (byte.wrapping_sub(48)) as i32;
        }
    }

    first.push(num1);
    second.push(num2);

    first.sort_unstable();
    second.sort_unstable();

    let mut total = Simd::splat(0);
    let mut first = first.chunks_exact(64);
    let mut second = second.chunks_exact(64);

    while let Some(first) = first.next() {
        // SAFETY: Violation of input format, let's just ignore that this function is not marked unsafe
        let second = unsafe { second.next().unwrap_unchecked() };

        let first: Simd<i32, 32> = Simd::from_slice(first);
        let second: Simd<i32, 32> = Simd::from_slice(second);

        total += (first - second).abs();
    }

    let total = total.reduce_sum();

    total
        + first
            .remainder()
            .iter()
            .zip(second.remainder())
            .map(|(f, s)| (f - s).abs())
            .sum::<i32>()
}

#[aoc(day1, part2)]
#[inline]
pub fn part2(input: &str) -> i32 {
    let input = input.as_bytes();

    let mut first = Vec::with_capacity(input.len() / 3);
    let mut second = IntMap::with_capacity_and_hasher(input.len() / 3, BuildNoHashHasher::default());

    let mut num1 = 0;
    let mut num2 = 0;
    let mut current = true;
    for &byte in input {
        if byte == b'\n' || byte == b'\r' {
            first.push(num1);
            *second.entry(num2).or_insert(0) += 1;

            num1 = 0;
            num2 = 0;
            current = true;

            continue;
        }
        if byte == b' ' {
            current = false;
            continue;
        }
        if current {
            num1 *= 10;
            num1 += (byte.wrapping_sub(48)) as i32;
        } else {
            num2 *= 10;
            num2 += (byte.wrapping_sub(48)) as i32;
        }
    }

    first.push(num1);
    *second.entry(num2).or_insert(0) += 1;

    first
        .iter()
        .map(|&n| n * *second.entry(n).or_default())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 11);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 31);
    }
}
