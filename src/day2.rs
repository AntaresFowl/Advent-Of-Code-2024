use aoc_runner_derive::aoc;

fn is_safe(iter: impl Iterator<Item = isize>) -> bool {
    let mut is_ascending = 0;
    let mut ascension_test_needed = true;
    iter.map_windows(|[level1, level2]| {
        let diff = level1 - level2;
        if ascension_test_needed {
            is_ascending = diff.signum();
            ascension_test_needed = false;
        }
        is_ascending == diff.signum() && diff.abs() <= 3 && diff.abs() != 0
    })
    .all(|is_safe| is_safe)
}

#[aoc(day2, part1)]
pub fn part1(input: &str) -> usize {
    let mut score = 0;
    score += input
        .lines()
        .map(|line| {
            is_safe(
                line.split_whitespace()
                    .map(str::parse::<isize>)
                    .map(Result::unwrap),
            )
        })
        .filter(|&is_safe| is_safe)
        .count();
    score
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
                    is_safe(line.split_whitespace()
                        .map(str::parse::<isize>)
                        .map(Result::unwrap)
                        .enumerate()
                        .filter(move |(pos, _)| i != *pos)
                        .map(|(_, level)| level))
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

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 4);
    }
}
