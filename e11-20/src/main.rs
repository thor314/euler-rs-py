// main struggle problems in this section were 11 and 18, and to some extent, 12 and 14. 17 was annoying to debug, but not hard.
extern crate timings_proc_macro;
use timings_proc_macro::timings;

#[timings]
fn e11() {
    let s: Vec<usize> = std::fs::read_to_string("src/e11.txt")
        .unwrap()
        .split_whitespace()
        .map(|n| n.parse::<usize>().unwrap())
        .collect();
    //println!("{:?}", s);
    // could just run with s, but let's build our 2d array.
    let mut v = [[0; 20]; 20];
    (0..400).for_each(|i| v[i / 20][i % 20] = s[i]);
    //println!("{:?}", v);
    let mut big = 0;
    use itertools::Itertools;
    (0..20).cartesian_product(0..20).for_each(|(i, j)| {
        if i < 17 {
            // h_
            let temp = v[i][j] * v[i + 1][j] * v[i + 2][j] * v[i + 3][j];
            if temp > big {
                // println!(
                //     "h_ new biggest: {} starting at: ({},{}), with init value {}:",
                //     big, i, j, v[i][j]
                // );
                big = temp
            }
        }
        if j < 17 {
            // v|
            let temp = v[i][j] * v[i][j + 1] * v[i][j + 2] * v[i][j + 3];
            if temp > big {
                // println!(
                //     "v| new biggest: {} starting at: ({},{}), with init value {}:",
                //     big, i, j, v[i][j]
                // );
                big = temp
            }
        }
        if i < 17 && j < 17 {
            // d\
            let temp = v[i][j] * v[i + 1][j + 1] * v[i + 2][j + 2] * v[i + 3][j + 3];
            if temp > big {
                // println!(
                //     "d\\ new biggest: {} starting at: ({},{}), with init value {}:",
                //     big, i, j, v[i][j],
                // );
                big = temp
            }
        }
        if i < 17 && j > 2 {
            // d/
            let temp = v[i][j] * v[i + 1][j - 1] * v[i + 2][j - 2] * v[i + 3][j - 3];
            if temp > big {
                // println!(
                //     "d/ new biggest: {} starting at: ({},{}), with init value {}:",
                //     big, i, j, v[i][j]
                // );
                big = temp
            }
        }
    });
    println!("biggest: {}", big);
}
// v elegant: https://github.com/zacharydenton/euler/blob/master/011/grid.rs
// 1. include_str!("grid.txt") I could be using this macro instead.
// 2.  .filter_map(|n| n.parse().ok()), well isn't that sweet.
// 3. his solution collects the maximum value in each direction in an interesting way. Each element is k farther ahead than the current elem. h:1,v:20,d\:21,d/:19. This fails if the line crosses a boundary though.

// What is the value of the first triangle number to have over five hundred divisors?
#[timings]
fn e12() {
    // entire problem is "count divisors". Naive soln sucks. Derive a soln.
    // Proposition. given X = p_1^a * p_2^b * ...,
    // N_factors(X) = (a+1)(b+1)....
    // now we only need to find the algebraic multiplicity of each prime divisor.
    let multiplicities = |input: usize| -> std::collections::HashMap<usize, usize> {
        let mut h = std::collections::HashMap::new();
        let mut n = input;
        while n % 2 == 0 {
            let counter = h.entry(2).or_insert(0);
            *counter += 1;
            n /= 2;
        }
        let mut i = 3;
        while n > 1 {
            while n % i == 0 {
                let counter = h.entry(i).or_insert(0);
                *counter += 1;
                n /= i;
            }
            i += 2;
        }

        h
    };

    let mut i = 1;
    let mut sum = 0;
    loop {
        sum += i;
        i += 1;
        let divisors = multiplicities(sum).values().fold(1, |acc, d| acc * (1 + d));
        //dbg!(sum, divisors);
        if divisors > 500 {
            println!("value: {}, the {}th triangle number", sum, i);
            break;
        }
    }
}

#[timings]
fn e13() {
    let s: Vec<String> = std::fs::read_to_string("src/e13.txt")
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<String>().unwrap())
        .collect();
    let s13: Vec<usize> = s
        .iter()
        .map(|l| l[..13].parse::<usize>().unwrap())
        .collect();

    let n = s13.iter().sum::<usize>().to_string();
    println!("e13: {}", &n[..10]);
}

#[allow(dead_code)]
fn collatz(n: usize) -> usize {
    match n % 2 {
        0 => n / 2,
        1 => 3 * n + 1,
        _ => unreachable!(),
    }
}
#[timings]
fn e14() {
    use std::collections::HashMap;
    let mut h = HashMap::new();
    h.insert(1, 0);

    let mut it_counter = 0;
    let mut biggest = (0, 0);
    for it in 2..1_000_000 {
        if h.contains_key(&it) {
            continue;
        }

        // Build a cache of values til we find a value we have seen
        let mut next = collatz(it);
        it_counter += 1;
        let mut cache: Vec<(usize, usize)> = vec![(it, it_counter)]; // 2: 1
        while h.get(&next).is_none() {
            it_counter += 1;
            cache.push((next, it_counter));
            next = collatz(next);
        }

        // the next value is now in the hashmap
        let count_last = *h.get(&next).unwrap();
        let count_for_it = count_last + it_counter;
        //println!("it:{},count: {}", it, count_for_it);

        for (n, c) in cache {
            let count = count_for_it + 1 - c;
            //println!("n:{},c: {}, count: {}", n, c, count);
            h.insert(n, count);
        }
        it_counter = 0;

        if count_for_it > biggest.0 {
            biggest = (count_for_it, it);
        }
    }
    println!("biggest seq len: {:?}, for n={:?}", biggest.0, biggest.1);
}

#[timings] //https://github.com/zacharydenton/euler/blob/master/014/collatz.rs
fn e14_zach_denton() {
    let mut collatz: Vec<usize> = vec![0; 1_000_000];
    collatz[1] = 1;
    let max = (2..collatz.len())
        .max_by_key(|&i| {
            let f = |n: usize| match n % 2 {
                0 => n / 2,
                _ => n * 3 + 1,
            };
            // og:
            let (mut j, mut len) = (i, 0);
            loop {
                // exit if:
                if j < collatz.len() && collatz[j] != 0 {
                    break;
                }
                len += 1;
                j = f(j);
            }
            len += collatz[j];
            collatz[i] = len;
            len
        })
        .unwrap();
    println!("{}", max);
}

// How many such (only move left or down) routes are there through a 20×20 grid?
#[timings]
fn e15() {
    // basic combinatorics. of 40 positions, choose 20. Equally, the 20th Catalan.
    let a: u128 = (21..=40).product();
    let b: u128 = (2..=20).product();
    println!("{}", a / b);
}

#[timings]
fn e16() {
    // mostly, futzing with bigint.
    use num_bigint::BigUint;
    // note that 2**1000 will have about 300 digits, so can't fit into a normal integer representation. Need a bigint.
    let a = BigUint::new(vec![2]);
    let b = a.pow(1000);
    //println!("{:?}", b);
    // TFAE:
    //let res = b.to_string().chars().fold(0, |a, d| a + d.to_digit(10).unwrap());
    let res: u32 = b.to_string().chars().map(|c| c.to_digit(10).unwrap()).sum();
    println!("{:?}", res);

    //let digits: num::BigInt = 2.pow(1000);
}

// If all the numbers from 1 to 1000 (one thousand) inclusive were written out in words, how many letters would be used?
#[timings]
fn e17() {
    let map = vec![
        (0, 0),
        (1, 3),
        (2, 3),
        (3, 5),
        (4, 4),
        (5, 4),
        (6, 3),
        (7, 5),
        (8, 5),
        (9, 4),
        (10, 3),
        (11, 6),
        (12, 6),
        (13, 8),
        (14, 8),
        (15, 7),
        (16, 7),
        (17, 9),
        (18, 8),
        (19, 8),
        (20, 6),
        (30, 6),
        (40, 5),
        (50, 5),
        (60, 5),
        (70, 7),
        (80, 6),
        (90, 6),
    ];
    let h = std::collections::HashMap::from_iter(map.into_iter());
    let res: usize = (1..=1000).fold(0, |acc, x| acc + count_letters(x, &h));
    println!("{}", res);
}
fn count_letters(d: usize, h: &std::collections::HashMap<usize, usize>) -> usize {
    let (a, b, c, e) = (d % 10, d / 10 % 10, d / 100 % 10, d / 1000 % 10);
    let aa = if b == 1 { 0 } else { *h.get(&a).unwrap() };
    let bb = if b == 1 {
        *h.get(&(b * 10 + a)).unwrap()
    } else {
        *h.get(&(b * 10)).unwrap()
    };
    let mut cc = if c > 0 { 3 + 7 + h.get(&c).unwrap() } else { 0 }; //  "and" counts apparently
    if c > 0 && aa == 0 && bb == 0 {
        cc -= 3 // 100 doesn't have an "and"
    };
    let ee = if e > 0 { 8 + h.get(&e).unwrap() } else { 0 };
    //println!("{}:{},{},{},{}", d, ee, cc, bb, aa);
    aa + bb + cc + ee
}

// first problem to be a bit of a challenge. I struggled picking a data structure and strategy for this one.
// A couple possible approaches occur:
// naive: at each step, pick the greatest next value
// brute: calculate the value of all 2^14 paths, not hard
// pruning: similar to brute, but if some sufficiently low sequence is included, exit early (optimization parameters: how often to prune, and what sufficiently low means)
// This problem begs to be solved recursively somehow.
#[timings]
fn e18() {
    let triangle: Vec<Vec<usize>> = std::fs::read_to_string("src/e18.txt")
        .unwrap()
        .lines()
        .map(|l| {
            l.split_whitespace()
                .into_iter()
                .map(|n| n.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect();
    let res = e18_less_naive_r(&triangle[1..], 75, 0);
    println!("{}", res);
}
/// traverse the triangle picking the greatest value at the next binary choice
#[allow(dead_code)]
fn e18_naive_r(t: &[Vec<usize>], running_sum: usize, last_index: usize) -> usize {
    if t.is_empty() {
        running_sum
    } else {
        let (rs, li) = if t[0][last_index] > t[0][last_index + 1] {
            (t[0][last_index], last_index)
        } else {
            (t[0][last_index + 1], last_index + 1)
        };
        println!("append:{},{}", rs, li);
        e18_naive_r(&t[1..], running_sum + rs, li)
    }
}
// 18 minutes to try naively. Now let's try a little harder.
// let's try something with look ahead.
const PEEK_DIST: usize = 5;
/// traverse the triangle picking the greatest single step-PEEK_DIST-chain at each next binary choice
fn e18_less_naive_r(t: &[Vec<usize>], running_sum: usize, last_index: usize) -> usize {
    if t.is_empty() {
        running_sum
    } else {
        // need to peek here
        let (_, dir, _path) = peek_ahead_r(t, running_sum, last_index, PEEK_DIST, None, vec![]);
        let (val, ind) = match dir {
            Dir::Left => (t[0][last_index], last_index),
            Dir::Right => (t[0][last_index + 1], last_index + 1),
        };
        //println!("append val:{}, ind:{}, path:{:?}", val, ind, _path);
        e18_less_naive_r(&t[1..], running_sum + val, ind)
    }
}

// if looking ahead 1 step, terminate, returning (running_sum, LEFT|RIGHT)
#[derive(Clone, Debug)]
enum Dir {
    Left,
    Right,
}
fn peek_ahead_r(
    t: &[Vec<usize>],
    running_sum: usize,
    last_index: usize,
    mut peek_dist: usize,
    first_step: Option<Dir>,
    /* debugging */ mut path: Vec<(usize, usize)>,
) -> (usize /* value */, Dir, Vec<(usize, usize)>) {
    if peek_dist > t.len() {
        peek_dist = t.len()
    }
    assert!(peek_dist > 0);
    if peek_dist == 1 {
        // if tie:  prefer rightward motion, THIS IS A (temporarily acceptable) BUG
        if t[0][last_index] > t[0][last_index + 1] {
            path.push((t[0][last_index], last_index));
            (
                t[0][last_index] + running_sum,
                first_step.unwrap_or(Dir::Left),
                path,
            )
        } else {
            path.push((t[0][last_index + 1], last_index + 1));
            (
                t[0][last_index + 1] + running_sum,
                first_step.unwrap_or(Dir::Right),
                path,
            )
        }
    } else {
        let mut p_left = path.clone();
        p_left.push((t[0][last_index], last_index));
        let left = peek_ahead_r(
            &t[1..],
            running_sum + t[0][last_index],
            last_index,
            peek_dist - 1,
            first_step.clone().unwrap_or(Dir::Left).into(),
            p_left,
        );
        let mut p_right = path.clone();
        p_right.push((t[0][last_index + 1], last_index + 1));
        let right = peek_ahead_r(
            &t[1..],
            running_sum + t[0][last_index + 1],
            last_index + 1,
            peek_dist - 1,
            first_step.unwrap_or(Dir::Right).into(),
            p_right,
        );
        if left.0 > right.0 {
            left
        } else {
            right
        }
    }
}

// How many Sundays fell on the first of the month during the twentieth century (1 Jan 1901 to 31 Dec 2000)?
#[timings]
fn e19() {
    // Sundays are uniformly distributed, with P(first is Sunday) = 1/7.
    // How many first of the months were there? 12*100
    println!("{}", 12.0 * 100.0 / 7.0);
}
// Can't win em all. But when ya do~

#[timings]
fn e20() {
    // Find the sum of the digits in the number 100!
    // would expect the number of digits to be roughly equiv to 50^100, which has about 150 digits, though there will of course be many zeroes, about 24. Still, it seems probably best to just shove it in a bigint. Anything more creative? 97 multiplications (2..99). Some theorem may exist about the sum of digits of the product of two numbers, could search for it. Meh, thought for 5 minutes, do the bigint thing.
    use num_bigint::BigUint;
    // note that 2**1000 will have about 300 digits, so can't fit into a normal integer representation. Need a bigint.
    let a = BigUint::new(vec![2]);
    let a = (3..=99).fold(a, |acc, i| acc * (i as u32));
    let res = a
        .to_string()
        .chars()
        .fold(0, |acc, i| acc + i.to_digit(10).unwrap());
    println!("{:?}", res);
}

fn main() {
    e11();
    e12();
    e13();
    //e14();
    e14_zach_denton();
    e15();
    e16();
    e17();
    e18();
    e19();
    e20();
}
