use std::{
    hint::assert_unchecked,
    ops::{BitAnd, Not},
    simd::{
        cmp::{SimdPartialEq, SimdPartialOrd},
        num::SimdUint,
        Simd,
    },
};

use aoc_runner_derive::aoc;
use memchr::memmem::find_iter;

unsafe fn to_num(bytes: &[u8]) -> usize {
    let len = bytes.len();
    debug_assert!(
        len >= 1 && len <= 3,
        "LEN WAS {} {:?}",
        bytes.len(),
        std::str::from_utf8(bytes).unwrap()
    );
    #[allow(clippy::manual_range_contains)]
    unsafe {
        assert_unchecked(len >= 1 && len <= 3);
    }

    let mut buffer = [b'0'; 4];
    buffer[3] = *unsafe { bytes.get_unchecked(len - 1) };
    if len >= 2 {
        buffer[2] = *unsafe { bytes.get_unchecked(len - 2) };
    }
    if len >= 3 {
        buffer[1] = *unsafe { bytes.get_unchecked(len - 3) };
    }

    let buffer = Simd::from_array(buffer) - Simd::splat(b'0');
    const MASK: Simd<u16, 4> = Simd::from_array([0, 100, 10, 1]);
    let buffer: Simd<u16, 4> = buffer.cast() * MASK;
    buffer.reduce_sum() as usize
}

#[aoc(day3, part1)]
fn part1(input: &str) -> usize {
    let input = input.as_bytes();
    let mut out = 0;

    let iter = find_iter(input, "mul(");
    for start_pos in iter {
        let start_pos = start_pos + 4;
        if input.len() >= start_pos + 8 {
            debug_assert!(
                input.len() >= start_pos + 8,
                "LENGTH OF INPUT {} REQUESTED {}",
                input.len(),
                start_pos + 8,
            );
            let buffer = unsafe { input.get_unchecked(start_pos..start_pos + 8) };
            let buffer: Simd<u8, 8> = Simd::from_slice(buffer);

            let separator_mask = buffer.simd_eq(Simd::splat(b','));
            let ending_bracket_mask = buffer.simd_eq(Simd::splat(b')'));
            let mut separator_pos = match separator_mask.first_set() {
                Some(pos) => pos,
                None => continue,
            };
            let mut ending_bracket_pos = match ending_bracket_mask.first_set() {
                Some(pos) => pos,
                None => continue,
            };

            let is_not_ascii_int = buffer
                .simd_ge(Simd::splat(b'0'))
                .bitand(buffer.simd_le(Simd::splat(b'9')))
                .not();
            match is_not_ascii_int.first_set() {
                Some(pos) => {
                    if pos != separator_pos && pos != ending_bracket_pos {
                        continue;
                    }
                }
                None => {}
            };
            separator_pos += start_pos;
            ending_bracket_pos += start_pos;

            let num1 = unsafe { input.get_unchecked(start_pos..separator_pos) };
            let num2 = unsafe { input.get_unchecked(separator_pos + 1..ending_bracket_pos) };

            let num1 = unsafe { to_num(num1) };
            let num2 = unsafe { to_num(num2) };

            out += num1 * num2;
            continue;
        } else {
            let mut i = start_pos;
            'outer: while i < input.len() {
                let byte = *unsafe { input.get_unchecked(i) };

                if byte == b',' {
                    let num1_end = i;
                    i += 1;
                    let num2_end = loop {
                        if i >= input.len() {
                            break 'outer;
                        }
                        let byte = *unsafe { input.get_unchecked(i) };

                        if byte == b')' {
                            break i;
                        }
                        if !byte.is_ascii_digit() {
                            continue 'outer;
                        }
                        i += 1;
                    };

                    let num1 = unsafe { input.get_unchecked(start_pos..num1_end) };
                    let num2 = unsafe { input.get_unchecked(num1_end + 1..num2_end) };

                    let num1 = unsafe { to_num(num1) };
                    let num2 = unsafe { to_num(num2) };

                    out += num1 * num2
                }
                i += 1;
            }
        }
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const INPUT2: &str = "<,:[*where()%mul(53,612)!^}&mul(3,518)??$~select()>??]mul(245,515),why()who()*@from()(where(242,190)mul(817,764)^select(),+who(851,301)where())from(){;mul(431,780)mul(110,982)what()what()]mul(441,829)??where()mul(269,112)>when()?who()<mul(131,147))}]what()^~)mul(186,137)when()'when(443,998)when()+-^what(770,821)?mul(742,949)>$**#!@mul(343,569),;what()from(){(;}mul(486,404)why()]#~when()%@do()+:'why(256,886)why()who(868,710)mul(103,406)?>mul(563,652)from()$-@when())!@:from()mul(744,992)[<^~}mul(822,789)],+select(45,52)!,why()<mul(609,728))who(879,937)&who(770,986):<-mul(723,41)<;select(366,476)why()(who()(%:'do()][@select()how(509,958)%^-<mul(402,767)]how()} #how();don't()how()mul(761,839)!%[#who()who()how()who(),-mul(769,131)>*mul(433,911) }why()<&&how()who()?]mul(877,899);':what())#{]@mul(401,705)@&<how()#mul(970,37)&# #+why()!how()mul(18,953)+what()mul(185,46)'/select()!{mul(474,848):>mul(627,54)? )~+mul(668,930)%)}do()what()#from()$/when()[:when() mul(545,444)how()]<how()^:?+-how()mul(470,601)how()~%from():how()mul(344,452)why()what(266,804)why()how(){who()do()mul(368,423)<#mul(662,80))+mul(7,851)mul(412,165)^,from()'*mul(653,405){}/,from()how()^#mul(592,415);}}>mul(409,150)>?%?@mul(295,205))'mul(321,875)mul(915,728){?when()what(644,532)*?;<<mul(61,812)#select()when()mul(101,342)<<!~why()mul(767,779)~#(^mul(176,413)}-]@>):(:mul(566,595)%'mul(499,468)/'where()}mul(721,340)]when()mul(162,291)mul(73,373)<]where()mul(100,385)][what()]mul(832,226)mul(675,546)who()how()#<^+how()'mul(119,723)what(){+,-mul(959,612)select()mul(758,905)!mul(247,521)&}$?don't()!'$ what()where(353,94)select(){mul(997,311)@from()/mul(987,583)&select(207,730)mul(299,379)select()do()what()select()when()select()]*?(?mul(841,179)!when()what()";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 161);
        assert_eq!(part1(INPUT2), 16082504);
    }
}
