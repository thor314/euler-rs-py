// useful references to other solutions
// https://github.com/zacharydenton/euler
// https://github.com/gifnksm/ProjectEulerRust/tree/master/src/bin
// https://github.com/dhbradshaw/ProjectEulerFastRust/blob/master/src/problems.rs
// https://github.com/groumache/project-euler-rust/blob/master/src/problem001to010.rs
extern crate timings_proc_macro;
use timings_proc_macro::timings;
#[timings]
fn e1() {
    let threes = 3 * 333 * 334 / 2;
    let fives = 5 * 199 * 200 / 2;
    let fifteens = 15 * 66 * 67 / 2;
    let out = threes + fives - fifteens;
    println!("{}", out);
}
#[timings]
fn e2() {
    let mut init = (1, 1);
    let f = |(a, b)| -> (u32, u32) { (a + b, a) };

    let mut sum = 0;
    while init.0 < 4_000_000 {
        if init.0 % 2 == 0 {
            sum += init.0;
        }
        init = f(init);
    }
    println!("{}", sum);
}
#[timings]
fn e3() {
    let mut x: u64 = 600851475143;
    let root_x = (x as f64).sqrt() as u64;
    let mut i = 3;
    let mut factors = vec![];
    while i < root_x && x > 1 {
        if x % i == 0 {
            factors.push(i);
            x /= i;
            i -= 2;
        }
        i += 2
    }
    println!("{:?}", factors);
}

fn is_pal(i: u32) -> bool {
    let s = i.to_string();
    s == s.chars().rev().collect::<String>()
}

fn e4() {
    let mut max = 0;
    for i in 100..=999 {
        for j in i..=999 {
            let prod = i * j;
            if is_pal(prod) && prod > max {
                max = prod;
            }
        }
    }
    println!("{}", max);
}

// What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?
#[timings]
fn e5() {
    let a = 19 * 17 * 16 * 13 * 11 * 9 * 7 * 5;
    println!("{}", a);
}
#[timings]
fn e6() {
    let sum_of_squares: u32 = (1..=100).map(|x| x * x).sum();
    let sum = (1..=100).sum::<u32>();
    let square_of_sums = sum * sum;
    println!(
        "diff of {} and {} is {}",
        sum_of_squares,
        square_of_sums,
        square_of_sums - sum_of_squares
    );
}
#[timings]
fn e7() {
    let mut count = 1;
    let mut primes: Vec<u32> = vec![2];
    let mut n: u32 = 3;
    while count < 10001 {
        let mut b = true;
        for p in &primes {
            if n % p == 0 {
                b = false;
                break;
            }
        }
        if b {
            primes.push(n);
            count += 1;
        }
        n += 2;
    }
    println!("{:?}", primes.last());
}
// review: could have used a sieve of eratosthenes
#[allow(dead_code)]
/// Return a vector of primes less than limit
fn eratosthenes(limit: usize) -> Vec<usize> {
    // https://github.com/zacharydenton/euler/blob/master/007/sieve.rs
    // initialize a vector of length `limit`
    let mut sieve = vec![true; limit];
    let mut p = 2;
    loop {
        // sieve all multiples of p
        let mut i = 2 * p - 1;
        while i < limit {
            sieve[i] = false;
            i += p;
        }
        // find the next prime p
        if let Some(n) = (p..limit).find(|&n| sieve[n] /* true */) {
            p = n + 1; // no off by one pls
                       //println!("next p: {}", p);
        } else {
            break;
        }
    }
    // remove all the dead values and return array
    let s = sieve
        .iter()
        .enumerate()
        .filter(|&(_, &is_prime)| is_prime)
        .skip(1)
        .map(|(i, _)| i + 1)
        .collect();
    //println!("values: {:?}", s);
    s
}
#[timings]
fn e8() {
    let s: Vec<u64> = std::fs::read_to_string("src/e8.txt")
        .unwrap()
        .replace("\n", "")
        .chars()
        .map(|c| c.to_string().parse().unwrap())
        .collect();
    //println!("{:?}", s);

    let mut max_prod = 0;
    for i in 0..=(s.len() - 13) {
        let v: Vec<u64> = s[i..(i + 13)].to_vec();

        let prod: u64 = v.iter().product();
        if prod > max_prod {
            max_prod = prod;
        }
    }
    println!("max prod: {}", max_prod);
}

/*
There exists exactly one Pythagorean triplet for which a + b + c = 1000.
Find the product abc.
 */
#[timings]
fn e9() {
    for a in 1..=1000 {
        for b in a..=1000 {
            for c in b..=1000 {
                if c * c == b * b + a * a {
                    //println!("pythagorean candidate: {}, {}, {}", a, b, c);
                    if a + b + c == 1000 {
                        println!(
                            "success! Pythagorean candidate found: {}, {}, {}, with product: {}",
                            a,
                            b,
                            c,
                            a * b * c
                        );
                    }
                }
            }
        }
    }
}
#[timings]
fn e9_better() {
    for a in 1..=1000 {
        for b in a..=1000 {
            let c = 1000 - a - b;
            if c * c == b * b + a * a {
                //println!("pythagorean candidate: {}, {}, {}", a, b, c);
                println!(
                    "success! Pythagorean candidate found: {}, {}, {}, with product: {}",
                    a,
                    b,
                    c,
                    a * b * c
                );
            };
        }
    }
}
#[timings]
fn e10() {
    let k: u64 = 2_000_000;
    let k_root: usize = (k as f64).sqrt() as usize;
    let mut r: Vec<u64> = (3..=k).step_by(2).collect();
    let (mut i, mut j): (usize, usize) = (0, 0);

    while i < r.len() && i < k_root {
        let p = r[i];
        i += 1;
        j = i;
        while j < r.len() {
            if r[j] % p == 0 {
                r.remove(j);
            }
            j += 1;
        }
    }
    //println!("r: {:?}", r);
    println!("sum: {:?}", 2 + r.iter().sum::<u64>());
}
#[timings]
fn e10_better() {
    // 300ms
    let limit = 2_000_000;
    let primes: Vec<usize> = eratosthenes(limit);
    println!("sum: {:?}", primes.iter().sum::<usize>());
}

fn main() {
    e1();
    e2();
    e3();
    e4();
    e5();
    e6();
    e7();
    e8();
    e9_better();
    e10_better();
}
