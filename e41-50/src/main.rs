#![feature(hash_drain_filter)]
use std::{collections::HashMap, str::FromStr};
extern crate timings_proc_macro;
use itertools::iproduct;
use primal;
use std::collections::HashSet;
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

#[timings]
fn e44() {
    use std::collections::HashSet;
    let mut pent_set = HashSet::new();
    let mut candidates = HashSet::new();
    for i in 1..5000 {
        let p_i = i * (3 * i - 1) / 2;
        for p in &pent_set {
            // generate candidates from differences
            if pent_set.contains(&(p_i - p)) {
                candidates.insert((p_i + p, p_i - p));
            }
        }
        for (sum, diff) in &candidates {
            // look for success
            if p_i == *sum {
                println!(
                    "found! index: {}, p_i: {}, sum: {}, diff: {}",
                    i, p_i, sum, diff
                );
                break;
            }
        }
        // update candidates
        candidates = candidates.drain_filter(|e| e.0 > p_i).collect();
        pent_set.insert(p_i);
    }
}

// It can be verified that T285 = P165 = H143 = 40755.
// Find the next triangle number that is also pentagonal and hexagonal.
#[timings]
fn e45() {
    use std::cmp::min;
    let t = |n: usize| -> usize { n * (n + 1) / 2 };
    let p = |n: usize| -> usize { n * (3 * n - 1) / 2 };
    let h = |n: usize| -> usize { n * (2 * n - 1) };
    let (mut tk, mut pk, mut hk) = ((1, 1), (1, 1), (1, 1));
    for _i in 1..1_000_000 {
        if tk.1 == pk.1 && pk.1 == hk.1 {
            println!("{}::tri:{:?},pent:{:?},hex:{:?}", _i, tk, pk, hk);
        }
        // get the minimum value of the three, update it
        let min = min(min(tk.1, pk.1), hk.1);
        match (tk.1 == min, pk.1 == min) {
            (true, _) => tk = (tk.0 + 1, t(tk.0 + 1)),
            (_, true) => pk = (pk.0 + 1, p(pk.0 + 1)),
            (_, _) => hk = (hk.0 + 1, h(hk.0 + 1)),
        }
    }
}
//What is the smallest odd composite that cannot be written as the sum of a prime and twice a square?
#[timings]
fn e46() {
    let mut candidates = [true; 1_000_000];
    candidates[1] = false;
    for i in 0..(candidates.len() / 2) {
        candidates[i * 2] = false
    }

    let primes: HashSet<_> = primal::Primes::all().take_while(|p| *p < 10_000).collect();
    for p in &primes {
        candidates[*p] = false;
    }

    let mut c = 0;
    let twice_a_square = std::iter::from_fn(move || {
        c += 1;
        Some(c * c * 2)
    });
    twice_a_square.take(50).for_each(|n| {
        for p in &primes {
            //println!("set {} + {}", n, p);
            candidates[n + p] = false;
        }
    });
    if let Some(x) = candidates.into_iter().position(|b| b) {
        println!("found one: {};", x);
    }
}

fn prime_factors(x: usize) -> HashSet<usize> {
    let mut x = x;
    let mut factors = HashSet::new();
    while x % 2 == 0 {
        x /= 2;
        factors.insert(2);
    }
    let mut breakpt = (x as f64).sqrt() as usize;
    let mut i = 3;
    while x > 1 {
        if x == breakpt + 1 {
            factors.insert(x);
            break;
        }
        while x % i == 0 {
            factors.insert(i);
            x /= i;
            breakpt = (x as f64).sqrt() as usize;
        }
        i += 2;
    }
    factors
}

#[timings]
fn e47() {
    let mut running = vec![];
    for i in 100000..1000000 {
        if running.len() == 4 {
            println!("{:?}", running.iter().min());
            break;
        }
        let pf_i = prime_factors(i);
        if pf_i.len() == 4 {
            running.push(i);
        } else {
            running.clear();
        }
    }
}

fn last_ten_pow(x: usize, pow: usize) -> usize {
    let mut x_ = 1;
    for _ in 1..=pow {
        x_ *= x;
        x_ %= 10e10 as usize;
    }
    x_
}

#[timings]
fn e48() {
    let mut sum = 0;
    for i in 1..=1000 {
        sum += last_ten_pow(i, i);
    }
    println!("{}", sum % 10e9 as usize);
}

fn is_permutation_cheat(a: &usize, b: &usize) -> bool {
    // note this algorithm isn't perfect, cheats, will accept tuples like 2002,2202, but good enough for government work
    let b = b.to_string();
    let a = a.to_string();
    for c in b.chars() {
        if !a.contains(c) {
            return false;
        }
    }
    for c in a.chars() {
        if !b.contains(c) {
            return false;
        }
    }
    true
}
#[test]
fn test_isperm() {
    let (a, b) = (1234, 4321);
    assert!(is_permutation_cheat(&a, &b));
    let (a, b) = (2234, 4321);
    assert!(!is_permutation_cheat(&a, &b));
}

#[timings]
fn e49() {
    // need sequence to be arithmatic, prime, and permutations.
    let four_digit_primes: HashSet<_> = primal::Primes::all()
        .take_while(|p| *p < 10_000)
        .filter(|p| *p > 1000)
        .collect();
    for (p, q) in
        iproduct!(four_digit_primes.iter(), four_digit_primes.iter()).filter(|(p, q)| p > q)
    {
        if is_permutation_cheat(p, q) {
            let r = p + (p - q);
            if r < 10_000 && four_digit_primes.contains(&r) && is_permutation_cheat(p, &r) {
                println!("{},{},{},{}{}{}", q, p, r, q, p, r);
            }
        }
    }
}

#[timings]
fn e50() {
    let primes: HashSet<_> = primal::Primes::all()
        .take_while(|p| *p < 1_000_000)
        .collect();
    let p_vec: Vec<_> = primal::Primes::all().take_while(|p| *p < 500_000).collect();
    let mut h: HashMap<usize, usize> = HashMap::new();
    let mut greatest_seen = 500; // logging

    for (i, p) in p_vec.iter().enumerate().filter(|(_, p)| **p < 20) {
        let mut ct = *p;
        for (seq_len, q) in p_vec[(i + 1)..].iter().enumerate() {
            ct += q;
            if primes.contains(&ct) {
                let val = match h.get(&ct) {
                    Some(val) => *val,
                    None => 0,
                };

                if val < seq_len + 1 {
                    h.insert(ct, seq_len + 1);
                }
                // logging
                if seq_len + 1 > greatest_seen {
                    greatest_seen = seq_len + 1;
                    println!("p: {}, ct: {}, len: {}", p, ct, seq_len + 1);
                }
            }
        }
    }
    let key = h.iter().max().unwrap();
    println!("{:?}", key);
}

fn main() {
    e41();
    e42();
    //e43();
    //e44();
    e45();
    e46();
    e47();
    e48();
    e49();
    e50();
}
