use aoc_runner_derive::aoc;

type Lists = (Vec<i32>, Vec<i32>);

#[inline]
fn parse_num(input: &[u8]) -> i32 {
    let mut out = 0;
    for num in input {
        out *= 10;
        out += (num - 48) as i32;
    }
    out
}

#[inline]
fn input_gen(input: &[u8]) -> Lists {
    let mut first = Vec::new();
    let mut second = Vec::new();

    for line in input.split(|&b| b == b'\n') {
        let mut position = 0;
        for byte in line {
            if byte == &b' ' {
                break;
            }
            position += 1;
        }

        // SAFETY: We know that this many bytes exist because we checked
        let f = unsafe { line.get_unchecked(..position) };
        // SAFETY: Violation of input format, let's just ignore that this function is not marked unsafe
        let s = unsafe { line.get_unchecked(position + 3..) };

        let f = parse_num(f);
        let s = parse_num(s);

        first.push(f);
        second.push(s);
    }

    (first, second)
}

#[aoc(day1, part1)]
fn part1(input: &[u8]) -> i32 {
    let (mut first, mut second) = input_gen(input);

    first.sort_unstable();
    second.sort_unstable();

    first.iter()
        .zip(second)
        .map(|(f, s)| (f - s).abs())
        .sum()
}

#[aoc(day1, part2)]
fn part2(input: &[u8]) -> i32 {
    let (first, second) = input_gen(input);
    first.iter()
        .map(|number| {
            number * second.iter().filter(|&num| num == number).count() as i32
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &[u8] = b"3   4
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
