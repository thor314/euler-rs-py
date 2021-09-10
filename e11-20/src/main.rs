// a procedural macro decorator to do similar stuff to my python decorator
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

fn main() {
    e11();
    e12();
    e13();
    e14();
}
