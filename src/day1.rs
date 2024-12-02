use std::simd::{num::*, Simd};

use aoc_runner_derive::aoc;
use gxhash::{HashMap, HashMapExt};

#[inline]
fn parse_num(input: &[u8]) -> i32 {
    let mut out = 0;
    for num in input {
        out *= 10;
        out += (num - 48) as i32;
    }
    out
}

#[aoc(day1, part1)]
#[inline]
pub fn part1(input: &str) -> i32 {
    let input = input.as_bytes();

    let mut first = Vec::with_capacity(input.len());
    let mut second = Vec::with_capacity(input.len());

    let mut prev_pos = 0;
    let mut pos = 0;

    while pos < input.len() {
        if input[pos] == b' ' {
            let next_pos = pos + 4 + (pos - prev_pos);

            // SAFETY: This is checked to be inside
            let f = unsafe { input.get_unchecked(prev_pos..pos) };
            // SAFETY: Goes against format
            let s = unsafe { input.get_unchecked(pos + 3..next_pos - 1) };

            let f = parse_num(f);
            let s = parse_num(s);

            first.push(f);
            second.push(s);

            prev_pos = next_pos;
            pos += 2;
        };
        pos += 1;
    }

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

    let mut first = Vec::with_capacity(input.len());
    let mut second = HashMap::with_capacity(input.len());

    let mut prev_pos = 0;
    let mut pos = 0;

    while pos < input.len() {
        if input[pos] == b' ' {
            let next_pos = pos + 4 + (pos - prev_pos);

            // SAFETY: This is checked to be inside
            let f = unsafe { input.get_unchecked(prev_pos..pos) };
            // SAFETY: Goes against format
            let s = unsafe { input.get_unchecked(pos + 3..next_pos - 1) };

            let f = parse_num(f);
            let s = parse_num(s);

            first.push(f);
            *second.entry(s).or_insert(0) += 1;

            prev_pos = next_pos;
            pos += 2;
        };
        pos += 1;
    }

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
