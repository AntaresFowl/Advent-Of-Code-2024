use std::i8;

use aoc_runner_derive::aoc;
use arrayvec::ArrayVec;

fn get_first_two_bytes(mut i: usize, input: &[u8]) -> (i8, bool, i8, usize) {
    let mut first_nums = [0_i8; 2];
    let mut addressor = false;
    let prev_i = i;
    while i < input.len() {
        let byte = *unsafe { input.get_unchecked(i) };
        if byte == b' ' {
            if addressor {
                i += 1;
                break;
            } else {
                addressor = true;
            }
        } else {
            first_nums[usize::from(addressor)] *= 10;
            first_nums[usize::from(addressor)] += byte as i8 - 48;
        }
        i += 1;
    }
    unsafe {
        std::hint::assert_unchecked(prev_i + 6 >= i);
    }
    let diff = first_nums[1] - first_nums[0];
    (
        diff.signum(),
        diff.abs() <= 3 && diff.abs() != 0,
        first_nums[1],
        i,
    )
}

#[aoc(day2, part1)]
pub fn part1(input: &str) -> usize {
    let input = input.as_bytes();
    let input = input.strip_prefix(b"\n").unwrap_or(input);
    let mut score = 0;
    let (mut is_ascending, mut is_safe, mut prev_num, mut j) = get_first_two_bytes(0, input);

    while j < input.len() - 1 {
        let byte = *unsafe { input.get_unchecked(j) };
        if byte == b'\n' {
            score += is_safe as usize;

            j += 1;
            let (ascending, safe, prev, offset) = get_first_two_bytes(j, input);
            j = offset;

            is_ascending = ascending;
            is_safe = safe;
            prev_num = prev;
            continue;
        }
        if byte == b' ' {
            j += 1;
            continue;
        }

        let convert = |x: u8| x as i8 - 48;
        let next = *unsafe { input.get_unchecked(j + 1) };
        let next_is_num = (next >= b'0' && next <= b'9') as i8;
        let num = (convert(byte) * 10 + convert(next)) * next_is_num + convert(byte);
        j += next_is_num as usize;

        let diff = num - prev_num;
        if is_safe && (is_ascending != diff.signum() || diff.abs() > 3) {
            is_safe = false;
        }

        prev_num = num;
        j += 1;
    }
    score + usize::from(is_safe)
}

fn is_safe(iter: impl Iterator<Item = i8>) -> bool {
    let mut is_ascending = 0;
    let mut ascension_test_needed = true;
    iter.map_windows(|[level1, level2]| {
        let diff = level1 - level2;
        if ascension_test_needed {
            if diff.signum() == 0 {
                return false;
            }
            is_ascending = diff.signum();
            ascension_test_needed = false;
        }
        is_ascending == diff.signum() && diff.abs() <= 3
    })
    .all(|is_safe| is_safe)
}

#[aoc(day2, part2)]
pub fn part2(input: &str) -> usize {
    let mut score = 0;
    let input = input.as_bytes();
    let input = input.strip_prefix(b"\n").unwrap_or(input);
    let lines = input.split(|&b| b == b'\n').map(|line| {
        line.split(|&b| b == b' ')
            .map(|bytes| {
                let mut num = 0;
                unsafe { std::hint::assert_unchecked(bytes.len() < 3) };
                for &byte in bytes {
                    num *= 10;
                    num += byte as i8 - 48;
                }
                num
            })
            .collect::<ArrayVec<_, 32>>()
    });

    for line in lines {
        if line.len() < 2 {
            break;
        }

        let mut line_is_safe = true;
        let mut is_ascending = 0;
        let mut ascension_test_needed = true;
        for i in 0..line.len() - 1 {
            let num = unsafe { line.get_unchecked(i) };
            let next = unsafe { line.get_unchecked(i + 1) };

            let diff = next - num;
            if ascension_test_needed {
                is_ascending = diff.signum();
                ascension_test_needed = false;
            }
            if is_ascending != diff.signum() || is_ascending == 0 || diff.abs() > 3 {
                if i == 1 {
                    line_is_safe = is_safe(
                        line.iter()
                            .enumerate()
                            .filter(|(j, _)| *j != 0)
                            .map(|(_, num)| *num),
                    );
                    if line_is_safe {
                        break;
                    }
                }

                line_is_safe = is_safe(
                    line.iter()
                        .enumerate()
                        .filter(|(j, _)| *j != i)
                        .map(|(_, num)| *num),
                );
                line_is_safe |= is_safe(
                    line.iter()
                        .enumerate()
                        .filter(|(j, _)| *j != i + 1)
                        .map(|(_, num)| *num),
                );
                if line_is_safe {
                    break;
                }
            }
        }
        score += usize::from(line_is_safe);
    }
    score
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
    const INPUT2: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
5 4 3 2 1
5 1 2 5 6
4 5 6 10 2
1 3 6 9 11";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 2);
        assert_eq!(part1(INPUT2), 4);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 4);
        assert_eq!(part2(INPUT2), 7);
        assert_eq!(part2(include_str!("../input/2024/day2.txt")), 536);
    }
}
