#![feature(destructuring_assignment)]
extern crate timings_proc_macro;
use num_bigint::BigUint;
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

fn main() {
    //e21();
    e22();
    e23();
    //e24();
    e25();
    e26();
}
