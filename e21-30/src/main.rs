#![feature(destructuring_assignment)]
extern crate timings_proc_macro;
//use num_bigint::BigUint;
use timings_proc_macro::timings;

fn is_amicable(a: usize) -> bool {
    let divisors = |n: usize| -> Vec<usize> { (1..=(n / 2)).filter(|x| n % x == 0).collect() };
    let divisors_sum = divisors(a).iter().sum();
    let other_div_sum = divisors(divisors_sum).iter().sum();
    //println!("{}:{},{}", a, divisors_sum, other_div_sum);
    if a == other_div_sum && a != divisors_sum {
        // debug a sneek snek
        //println!("{}:,{},{}", a, divisors_sum, other_div_sum);
        return true;
    }
    false
}
//Evaluate the sum of all the amicable numbers under 10000.
#[timings]
fn e21() {
    let arr: Vec<usize> = (1..10000)
        .into_iter()
        .map(|i| (i, is_amicable(i)))
        .filter(|(_i, b)| *b)
        .map(|(i, _)| i)
        .collect();

    println!("{:?},{}", arr, arr.iter().sum::<usize>());
} // A recursive solution would have been significantly faster.

#[timings]
fn e22() {
    let mut arr: Vec<String> = include_str!("p022_names.txt")
        .replace("\"", "")
        .split(',')
        .map(|s| s.to_string())
        .collect();
    arr.sort();
    let s = "0ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let score_c = |c| -> usize { s.find(c).unwrap() };
    let score_name = |pos: usize, name: String| -> usize {
        (1 + pos) * name.chars().map(score_c).sum::<usize>()
    };
    println!(
        "{}",
        arr.into_iter()
            .enumerate()
            .map(|(i, name)| score_name(i, name))
            .sum::<usize>()
    );
}

#[timings]
fn e23() {
    const STOP: usize = 28124;
    let mut arr = [0; STOP];
    for i in 1..STOP {
        for j in ((i * 2)..STOP).step_by(i) {
            arr[j] += i
        }
    }
    //println!("{:?}", &arr[..40]);
    let abundants: Vec<usize> = arr
        .iter()
        .enumerate()
        .filter(|(i, v)| i < v)
        .map(|(i, _v)| i)
        .collect();
    //println!("{:?}", &abundants[..40]);
    let mut is_ab_sum = [false; STOP];
    for i in &abundants {
        for j in &abundants {
            if i + j < STOP {
                is_ab_sum[i + j] = true;
            } else {
                break; //inner j
            }
        }
    }
    let out: usize = is_ab_sum
        .into_iter()
        .enumerate()
        .filter(|(_i, v)| !v)
        .map(|(i, _v)| i)
        .sum();
    println!("{}", out);
}

//What is the millionth lexicographic permutation of the digits 0, 1, 2, 3, 4, 5, 6, 7, 8 and 9?
#[timings]
fn e24() {
    use itertools::Itertools;
    let s = (0..=9).permutations(10).nth(1_000_000 - 1).unwrap();
    let s_ = s.iter().join("").parse::<u64>().unwrap();
    println!("{:?}", s_);
}

#[timings]
fn e25() {
    use num_bigint::BigUint;
    let mut a = BigUint::new(vec![1]);
    let mut b = BigUint::new(vec![1]);
    let mut counter_a = 2;
    while a.to_string().len() < 1000 {
        counter_a += 1;
        let temp = a.clone();
        a += b;
        b = temp;
    }
    println!("{}", counter_a);
}

//Find the value of d < 1000 for which 1/d contains the longest recurring cycle in its decimal fraction part.

// What are the cases?
// 0. 1/d terminates (2,4,5...)
// 1. 1/d immediately 1-cycles the same digit (3,9....)
// 2. 1/d begins to cycle after one or more non-cycle digits (6)
// 3. 1/d cycles n-cycles, where n<17 (7:6-cycle)
// 4. 1/d cycles n-cycles, where n>17 (38 maybe?)
// case 4 totally screws my f64's. What do I do? get a bigger float?
// suppose I had arbitrarily long floats. How would I find the cycles?
// Problems with the imperfect cycle implementation:
// 1. I'm not guaranteed the first nonzero digit is the start of the cycle.
// 2. I'm not guaranteed the first re-appearance of that digit signals the end of the cycle.
// So, I need 2 new features:
// 1. Look at the first n (try n=5) nonzero digits, and look for the re-appearance of that digit later in the float.
// 2. Don't just take the the first digit, take the first m-slice (try m=3) after each initial digit.
// Zeroeth, validate we can find an arbitrary float crate. Okay, I can get 100 decimal point precision. Suppose I need more? For any number that looks to overrun 100 decimal points, I could use big_int, and then add 10k zeroes behind each value before division (lol, it turns out this breaks bigints. Yes of course I tried it.)
//
// don't laugh. You laughed. Why did you laugh.
// I need like a thousand decimal points. That seems...impractical. This problem is "easy?"
//
// I'm missing something. I'm missing some elegant strategy. What could I do to make this "easy"?
// I could try incremental long division. I guess that's where we're at.

// fn _e26() {
//     let mut s = "".to_string();
//     let divisor = 101;
//     let (mut quotient, mut remainder) = (0, 1);
//     for _ in 1..10 {
//         (quotient, remainder) = next_long_div_step(remainder, divisor);
//         s.push(char::from_digit(quotient, 10).unwrap()); // add a digit to s

//         // check s for repeats
//         let repeat = check_repeats(&s);
//         if repeat.0 {
//             println!("repeat:{}", &s[repeat.1..repeat.1 + 3]);
//         }
//     }
//     println!("{}", s);
// }
#[timings]
fn e26() {
    let mut greatest = (0, 0);
    for divisor in 2..1000usize {
        let mut quotient;
        // init remainder: greatest power of 10 such that divisor > remainder
        let mut remainder: usize = 10usize.pow((divisor - 1).to_string().len() as u32);
        let mut s = "".to_string();
        let mut repeat = Repeats(false, 0);
        while !repeat.0 {
            (quotient, remainder) = next_long_div_step(remainder, divisor);
            remainder *= 10;
            if remainder == 0 {
                break;
            }
            s.push(char::from_digit(quotient as u32, 10).unwrap()); // add a digit to s

            // check s for repeats
            repeat = check_repeats(&s);
            if repeat.0 {
                if greatest.1 < repeat.1 {
                    //println!("greatest update: {},{}", divisor, repeat.1,);
                    greatest.0 = divisor;
                    greatest.1 = repeat.1;
                }
            }
        }
    }
    println!("{:?}", greatest.0);
}

fn next_long_div_step(
    remainder: usize,
    divisor: usize,
) -> (/*quotient*/ usize, /*remainder*/ usize) {
    (remainder / divisor, remainder % divisor)
}

struct Repeats(bool, usize);
fn check_repeats(s: &str) -> Repeats {
    const M_SLICE_LEN: usize = 3;
    if s.len() < M_SLICE_LEN + 1 {
        return Repeats(false, 0);
    }
    // if slice is of len 5, compare only 5-M_SLICE_LEN=2 chunks
    let n_chunks: usize = std::cmp::min(10, s.len() - M_SLICE_LEN);
    for i in 0..n_chunks {
        let chunk = &s[i..(i + M_SLICE_LEN)];
        let last_idx = s.len() - M_SLICE_LEN;
        let last_chunk = &s[last_idx..(last_idx + M_SLICE_LEN)];
        if chunk == last_chunk {
            //println!("chunk:{},last_idx:{}", chunk, last_idx);
            return Repeats(true, last_idx);
        }
    }
    Repeats(false, 0)
}

//Find the product of the coefficients, a and a, for the quadratic expression that produces the maximum number of primes for consecutive values of n, starting with n=0.
#[timings]
fn e27() {
    let mut arr: Vec<((isize, isize), isize)> = vec![];
    for a in -1000..1000 {
        //for a in -10..10 {
        for b in (std::cmp::max(2, a)..1000isize)
            .into_iter()
            // premature optimization corner :D
            .filter(|b| is_prime(*b) && is_prime(1 + a + b))
        {
            arr.push(((a, b), e27_prime_seq_len(2, a, b, 2)));
        }
    }
    arr.sort_by(|a, b| b.1.cmp(&a.1));
    println!(
        "{:?},product:{}; tuples checked:{}",
        &arr[0],
        arr[0].0 .0 * arr[0].0 .1,
        arr.len()
    );
}
fn e27_prime_seq_len(n: isize, a: isize, b: isize, counter: isize) -> isize {
    let next = n * n + a * n + b;
    if is_prime(next) {
        //println!("n={},a={},b={}::{} was prime", n, a, b, next);
        e27_prime_seq_len(n + 1, a, b, counter + 1)
    } else {
        counter
    }
}

fn is_prime(n: isize) -> bool {
    match &n.cmp(&2) {
        std::cmp::Ordering::Less => false,
        std::cmp::Ordering::Equal => true,
        std::cmp::Ordering::Greater => {
            if n % 2 == 0 {
                return false;
            }
            let lim = ((n as f64).sqrt() + 1.0) as isize;
            for i in (3..lim).step_by(2) {
                if n % i == 0 {
                    //println!("{} not prime by: {}", n, i);
                    return false;
                }
            }
            true
        }
    }
}

#[test]
fn e27_test() {
    assert_eq!(e27_prime_seq_len(0, 1, 41, 0), 40);
    assert_eq!(e27_prime_seq_len(0, -79, 1601, 0), 80);
    assert!(is_prime(2));
    assert!(is_prime(5));
    assert!(is_prime(47));
    assert!(!is_prime(-47));
}

#[timings]
fn e28() {
    let s = 1001;
    let v = e28_gen_array(s);
    let row_sums = (0..s)
        .map(|i| {
            if i == (s - 1) / 2 {
                v[i][i] // center
            } else {
                v[i][i] + v[i][s - i - 1] // eg: row 0: want [0][0], [0][1000]
            }
        })
        .collect::<Vec<usize>>();
    println!("{}", row_sums.iter().sum::<usize>());
}

fn e28_gen_array(size: usize) -> Vec<Vec<usize>> {
    assert!(size % 2 == 1);
    let arr = vec![vec![0; size]; size];
    let arr_gen = e28_gen_r(arr, size);
    //println!("{:?}", arr_gen);
    arr_gen
}

enum Dir {
    Right,
    Left,
    Up,
    Down,
}
fn e28_gen_r(mut arr: Vec<Vec<usize>>, size: usize) -> Vec<Vec<usize>> {
    let (mut i, mut j) = (size / 2, size / 2);
    let mut count = 1;
    //println!("{},{}: {}", i, j, count);

    loop {
        if arr[i][j] != 0 {
            panic!("arr[{}][{}] haz val:{}", i, j, arr[i][j]);
        }
        arr[i][j] = count;
        count += 1;

        if (i, j) == (0, size - 1) {
            return arr;
        } else {
            let dir = match i.cmp(&j) {
                std::cmp::Ordering::Equal => {
                    if i <= (size - 1) / 2 {
                        Dir::Right
                    } else {
                        Dir::Left
                    }
                }
                std::cmp::Ordering::Less => {
                    if i + j >= size {
                        Dir::Down
                    } else {
                        Dir::Right
                    }
                }
                std::cmp::Ordering::Greater => {
                    if i + j > size - 1 {
                        Dir::Left
                    } else {
                        Dir::Up
                    }
                }
            };

            match dir {
                Dir::Right => j += 1,
                Dir::Left => j -= 1,
                Dir::Up => i -= 1,
                Dir::Down => i += 1,
            }
        }
    }
}

#[test]
fn test_gen_arr() {
    let arr = e28_gen_array(3);
    assert_eq!(arr[1][1], 1);
    assert_eq!(arr[0][2], 9);
    let arr = e28_gen_array(5);
    assert_eq!(arr[0][0], 21);
    assert_eq!(arr[0][4], 25);
    assert_eq!(arr[4][0], 17);
    assert_eq!(arr[4][4], 13);
    assert_eq!(arr[2][2], 1);
}

#[timings]
fn e29() {
    // How many distinct terms are in the sequence generated by a^b for 2 ≤ a ≤ 100 and 2 ≤ b ≤ 100?
    use itertools::{iproduct, Itertools};
    use num::{bigint::Sign, BigInt};
    let mut r = iproduct!(2..=100, 2..=100)
        .into_iter()
        .map(|(a, b)| {
            let a: BigInt = BigInt::new(Sign::Plus, vec![a]);
            a.pow(b)
        })
        .collect::<Vec<BigInt>>();
    r.sort();
    let uniques: Vec<_> = r.iter().unique().collect();
    let len = uniques.len();
    println!("{}", len);
}

#[timings]
fn e30() {
    let results = (2..590_490usize)
        .into_iter()
        .filter(|n| is_fifth_pow_sum(*n))
        .collect::<Vec<usize>>();
    println!("{},{:?}", results.iter().sum::<usize>(), results);
}
fn is_fifth_pow_sum(n: usize) -> bool {
    let dig_v: Vec<char> = n.to_string().chars().collect();
    let sum: usize = dig_v
        .iter()
        .map(|c| (c.to_digit(10).unwrap()).pow(5) as usize)
        .sum();
    sum == n
}

fn main() {
    e21();
    e22();
    e23();
    e24();
    e25();
    e26();
    e27();
    e28();
    e29();
    e30();
}
