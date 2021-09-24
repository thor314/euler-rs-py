#![feature(drain_filter)]
extern crate timings_proc_macro;
use itertools::Itertools;
use primal;
use timings_proc_macro::timings;

#[timings]
fn e31() {
    let coins = vec![200, 100, 50, 20, 10, 5, 2, 1];
    let amt = 200;
    println!("{}", num_ways_sum(coins, amt));
}
#[allow(dead_code)]
fn num_ways_sum(arr: Vec<usize>, total: usize) -> usize {
    if total == 0 {
        1
    } else {
        let mut fin = 0;
        let arr_ = arr
            .into_iter()
            .filter(|&x| x <= total)
            .collect::<Vec<usize>>();
        for (i, coin) in arr_.iter().enumerate() {
            fin += num_ways_sum(arr_[i..].to_vec(), total - coin);
        }
        fin
    }
}

//Find the sum of all products whose multiplicand/multiplier/product identity can be written as a 1 through 9 pandigital.
#[timings]
fn e32() {
    // a * b = product
    // n_digits(product) >= max(n_digits(a),n_digits(b)) => max_n_digits(a) <= 4
    // all values will have form: abcd*e=fghi, where a*e<10 OR abc*de=fghi, where a*d<10
    // lazy way: generate all 9! permutations, *2 possible placements for symbols, filter by is_pandigital.
    // iterative way: a in range 100..9999,b in range 1..99 s.t. a*b<10000 and digits of b not repeated in a. Check pandigital. Seems likely more efficient.
    let mut set = HashSet::new();
    for a in (102..=9876)
        .into_iter()
        .filter(|a| unique_digits_not_zero(*a)) // digits of a are unique
        .collect::<Vec<_>>()
    {
        let digits_a = digits(a);
        for b in (1..=98).into_iter().filter(|b| {
            b % 11 != 0 && a * b < 10_000 && digits_a.intersection(&digits(*b)).next().is_none()
        }) {
            let c = a * b;
            let digits_b = digits(b);
            let digits_c = digits(c);
            if unique_digits_not_zero(c)
                && format!("{}{}{}", a, b, c).len() == 9
                && digits_c.intersection(&digits_a).next().is_none()
                && digits_c.intersection(&digits_b).next().is_none()
            {
                //println!("insert:{}*{}={}", a, b, a * b);
                set.insert(c);
            }
        }
    }

    println!("{},{:?}", set.iter().sum::<usize>(), set);
}
fn unique_digits_not_zero(n: usize) -> bool {
    let digits_n = digits(n);
    n.to_string().len() == digits_n.iter().unique().count() && !digits_n.contains(&0)
}
fn digits(n: usize) -> HashSet<u32> {
    n.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect()
}

// get the product of the four curious fractions of 2 digits
// might be less efficient, but I haven't defined any structs over generics in awhile. Could be nice to practice a bit.
#[timings]
fn e33() {
    let v: Vec<_> = (11..=99)
        .into_iter()
        .flat_map(|i| {
            (i..=99)
                .filter(move |j| is_curious(i, *j))
                .map(move |j| Fraction(i, j))
        })
        .collect();
    let product: Fraction<usize> = v
        .into_iter()
        .inspect(|f| println!("f: {:?}", f))
        .fold(Fraction(1, 1), |acc, i| acc * i);
    println!("{:?}", product.reduce());
}
// LET'S GET INEFFICIENT
use num::Integer;
use std::collections::HashSet;
use std::ops::Mul;
use std::str::FromStr;
#[derive(Debug)]
struct Fraction<T: Mul>(T, T);
impl<T: Mul + Copy + Integer + Mul<Output = T>> Mul for Fraction<T> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Fraction(self.0 * rhs.0, self.1 * rhs.1).reduce()
    }
}
impl<T: Integer + Mul + Copy> Fraction<T> {
    fn reduce(self) -> Self {
        let gcd = self.0.gcd(&self.1);
        Self(self.0 / gcd, self.1 / gcd)
    }
}
fn is_curious<T: Copy + Mul + Integer + std::fmt::Display + std::str::FromStr>(a: T, b: T) -> bool
where
    <T as FromStr>::Err: std::fmt::Debug,
{
    let digits_a: HashSet<char> = a.to_string().chars().collect();
    let digits_b: HashSet<char> = b.to_string().chars().collect();
    let intersect: HashSet<&char> = digits_a.intersection(&digits_b).collect();
    if intersect.is_empty() {
        false
    } else {
        if intersect.contains(&'0') {
            return false;
        }
        let mut da = digits_a.clone();
        let mut db = digits_b.clone();
        intersect.into_iter().for_each(|c| {
            da.remove(c);
            db.remove(c);
        });
        if !da.is_empty() && !db.is_empty() {
            let a_ = da.into_iter().join("").parse::<T>().unwrap();
            let b_ = db.into_iter().join("").parse::<T>().unwrap();
            let curiously = Fraction(a_, b_).reduce();
            let reduced = Fraction(a, b).reduce();
            reduced.0 == curiously.0 && reduced.1 == curiously.1
        } else {
            false
        }
    }
}

//Find the sum of all numbers which are equal to the sum of the factorial of their digits.
#[timings]
fn e34() {
    // upper limit: 9!=362880; ergo 9_999_999=>362_880*7=2_1xx_xxx. Therefore we can easily stop at about 2.2mil, and probably much less.
    let facs: Vec<usize> = (0..=9).map(fac).collect();
    let lim = 2_200_000usize;
    let v: Vec<usize> = (3..lim)
        .filter(|&n| e34_digits(n).iter().map(|&k| facs[k]).sum::<usize>() == n)
        .collect();
    println!("{},{:?}", v.iter().sum::<usize>(), v);
    // kindof inefficient, took 5s, how could faster? Well the only two values are 145 and 40585, so my upper limit is a massive 50x overshot
}
#[allow(dead_code)]
fn e34_digits(n: usize) -> Vec<usize> {
    n.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect()
}
#[allow(dead_code)]
fn fac(n: usize) -> usize {
    if n == 1 || n == 0 {
        1
    } else {
        n * fac(n - 1)
    }
}

#[timings]
fn e35() {
    use std::collections::HashSet;
    let primes: HashSet<usize> = eratosthenes(1_000_000);
    //println!("{:?}", &primes[..10]);
    let set: HashSet<&usize> = primes.iter().filter(|&n| circular(*n, &primes)).collect();
    println!("{},{:?}", set.len(), set);
}
fn eratosthenes(limit: usize) -> HashSet<usize> {
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
    sieve
        .iter()
        .enumerate()
        .filter(|&(_, &is_prime)| is_prime)
        .skip(1)
        .map(|(i, _)| i + 1)
        .collect()
}
fn circular(n: usize, p: &HashSet<usize>) -> bool {
    if !p.contains(&n) {
        false
    } else {
        let mut s = n.to_string();
        for _i in 0..s.len() {
            s = format!("{}{}", &s[1..], &s[0..1]);
            if !p.contains(&s.parse::<usize>().unwrap()) {
                return false;
            }
        }
        //println!("{} is circular", n);
        true
    }
}
#[test]
fn test_circular() {
    let p = eratosthenes(1000);
    assert!(circular(197, &p));
    assert!(!circular(198, &p));
    assert!(!circular(201, &p));
}

//Find the sum of all numbers, less than one million, which are palindromic in base 10 and base 2.
#[timings]
fn e36() {
    let v: Vec<usize> = (1..=1_000_000)
        .into_iter()
        .filter(is_pal_dec)
        .filter(is_pal_bin)
        .collect();
    println!("{},{:?}", v.iter().sum::<usize>(), v);
}
fn is_pal_dec(n: &usize) -> bool {
    let s = n.to_string();
    s == s.chars().rev().collect::<String>()
}
fn is_pal_bin(n: &usize) -> bool {
    let b = to_binary(n);
    b == b.iter().cloned().rev().collect::<Vec<u8>>()
}
fn to_binary(n: &usize) -> Vec<u8> {
    format!("{:b}", n)
        .chars()
        .map(|c| c.to_digit(2).unwrap() as u8)
        //.inspect(|d| println!("{}", d))
        .collect()
}
#[test]
fn test_e36() {
    assert_eq!(to_binary(&585), vec!(1, 0, 0, 1, 0, 0, 1, 0, 0, 1));
    assert_ne!(to_binary(&587), vec!(1, 0, 0, 1, 0, 0, 1, 0, 0, 1));
    assert!(is_pal_bin(&585));
    assert!(!is_pal_bin(&587));
    assert!(is_pal_dec(&585));
    assert!(!is_pal_dec(&587));
}

//Find the sum of the only eleven primes that are both truncatable from left to right and right to left. (exclude 2-7)
#[timings]
fn e37() {
    // while v is less than
    let mut primes = HashSet::new();
    let mut v = vec![];
    let mut sieve = primal::Primes::all();

    while v.len() < 11 {
        let p = sieve.next().unwrap() as usize;
        primes.insert(p);
        if is_prime_truncatable(p, &primes) {
            v.push(p);
        }
    }
    println!("{},{:?}", v.iter().sum::<usize>(), v);
}
fn is_prime_truncatable(n: usize, p: &HashSet<usize>) -> bool {
    if [2, 3, 5, 7].contains(&n) {
        return false;
    }
    let mut nn = n;
    let mut nm = n;
    while nn > 0 {
        if !p.contains(&nn) || !p.contains(&nm) {
            return false;
        }
        nn /= 10;
        nm %= 10usize.pow(nm.to_string().len() as u32 - 1);
        //println!("{}:{},{}", n, nm, nn);
    }
    true
}
#[test]
fn test_e37() {
    let p = eratosthenes(4000);
    assert!(is_prime_truncatable(3797, &p));
    assert!(!is_prime_truncatable(3897, &p));
}

#[timings]
fn e38() {
    let v = (1..10_000)
        .into_iter()
        .map(e38_expand)
        .filter(|&n| is_pandigital(n))
        .collect::<Vec<usize>>();
    println!("{},{:?}", v.iter().max().unwrap(), v);
}

fn e38_expand(n: usize) -> usize {
    let mut m = 0;
    let mut counter = 1;
    while m.to_string().len() < 9 {
        let temp = n * counter;
        m *= 10usize.pow(temp.to_string().len() as u32);
        m += temp;
        counter += 1;
    }
    m
}
fn is_pandigital(n: usize) -> bool {
    let s: HashSet<_> = HashSet::from_iter("123456789".to_string().chars());
    s.intersection(&HashSet::from_iter(n.to_string().chars()))
        .count()
        == 9
        && n.to_string().len() == 9
}
#[test]
fn test_e38() {
    assert!(is_pandigital(192384576));
    assert!(!is_pandigital(292384576));
    assert_eq!(192384576, e38_expand(192));
}

fn main() {
    //e31();
    e32();
    e33();
    //e34();
    e35();
    e36();
    e37();
    e38();
}
