use aoc_runner_derive::aoc;

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
            first_nums[addressor as usize] *= 10;
            first_nums[addressor as usize] += byte as i8 - 48;
        }
        i += 1;
    }
    debug_assert!(prev_i + 6 >= i, "{prev_i} {i} {:?}", std::str::from_utf8(&input[prev_i..i]).unwrap());
    unsafe { std::hint::assert_unchecked(prev_i + 6 >= i); }
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
        let byte = input[j];
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

        let convert = |x: u8| { x as i8 - 48 };
        let mut num = convert(byte);
        if input[j + 1] >= b'0' && input[j + 1] <= b'9' {
            num *= 10;
            num += convert(input[j + 1]);
            j += 1;
        }

        let diff = num - prev_num;
        if is_safe && (is_ascending != diff.signum() || diff.abs() > 3) {
            is_safe = false;
        }

        prev_num = num;
        j += 1;
    }
    score + is_safe as usize
}

fn is_safe(iter: impl Iterator<Item = isize>) -> bool {
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
    score += input
        .lines()
        .map(|line| {
            is_safe(
                line.split_whitespace()
                    .map(str::parse::<isize>)
                    .map(Result::unwrap),
            ) || (0..line.len() / 2)
                .map(|i| {
                    is_safe(
                        line.split_whitespace()
                            .map(str::parse::<isize>)
                            .map(Result::unwrap)
                            .enumerate()
                            .filter(move |(pos, _)| i != *pos)
                            .map(|(_, level)| level),
                    )
                })
                .any(|is_safe| is_safe)
        })
        .filter(|&is_safe| is_safe)
        .count();
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
    }
}
