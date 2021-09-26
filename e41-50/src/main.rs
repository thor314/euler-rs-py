use std::str::FromStr;
extern crate timings_proc_macro;
use std::collections::HashSet;

use itertools::Itertools;
use primal;
use timings_proc_macro::timings;

// What is the largest n-digit pandigital prime that exists?
#[timings]
fn e41() {
    // we would expect the largest n-digit pandigital to have 9 digits or fewer. The problem of this algorithm is that we don't know what the question asks: Whether 1-0 is pandigital, or only 1-9.
    // naive: collect primes 1..10^9, filter them by pandigital, get the greatest.
    // less naive? I could get the pandigitals first and filter them by prime.
    // perhaps an oracle attack? Try starting at 10^7 and attack the PE oracle.
    // oracle attack was very effective!
    const LIMIT: usize = 10_000_000;
    let primes = primal::Primes::all().take_while(|p| *p < LIMIT);
    let pandigital_primes: Vec<_> = primes.filter(|n| is_pandigital(*n)).collect();
    let m = pandigital_primes.iter().max().unwrap();
    println!("{}", m);
    //println!("{}, {:?}", m, pandigital_primes);
}

fn is_pandigital(n: usize) -> bool {
    if n > 1_000_000_000 {
        panic!("{} too big", n);
    }
    let s = n.to_string();
    let size = s.len();
    for i in 1..=size {
        if !s.contains(char::from_digit(i as u32, 10).unwrap()) {
            //println!("{}: {} witnesses non-pandigital", n, i);
            return false;
        }
    }
    true
}

#[timings]
fn e42() {
    let arr: Vec<String> = include_str!("p042_words.txt")
        .replace("\"", "")
        .split(',')
        .map(|s| s.to_string())
        .collect();

    let tris: HashSet<_> = HashSet::from_iter((0..30).map(|i| i * (i + 1) / 2));
    let twords: Vec<_> = arr
        .iter()
        .filter(|word| tris.contains(&score(word)))
        .collect();
    println!("{}", twords.len());
}
fn score(n: &str) -> usize {
    let mut tot = 0;
    for c in n.chars() {
        tot += c as usize - 64;
    }
    tot
}

#[timings]
fn e43() {
    use itertools::Itertools;
    let arr = [2, 3, 5, 7, 11, 13];
    let special_pans: Vec<usize> = (0..=9)
        .permutations(10)
        .filter(|v| (v[7] * 100 + v[8] * 10 + v[9]) % 17 == 0)
        .map(digits_to_int)
        .filter(|&n| is_special(n, &arr))
        .collect();
    println!("{},{:?}", special_pans.iter().sum::<usize>(), special_pans);
}
fn digits_to_int(v: Vec<usize>) -> usize {
    let mut n = 0;
    for k in v.into_iter() {
        n = n * 10 + k;
    }
    n
}
fn is_special(n: usize, arr: &[usize]) -> bool {
    for (i, v) in arr.iter().enumerate() {
        let m = usize::from_str(&n.to_string()[(i + 1)..(i + 4)]).unwrap();
        if m % v != 0 {
            return false;
        }
    }
    true
}

fn main() {
    e41();
    e42();
    e43();
}
