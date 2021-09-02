fn e1() {
    let threes = 3 * 333 * 334 / 2;
    let fives = 5 * 199 * 200 / 2;
    let fifteens = 15 * 66 * 67 / 2;
    let out = threes + fives - fifteens;
    println!("{}", out);
}

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

fn main() {
    let now = std::time::Instant::now();
    e1();
    println!("time:{:?}", now.elapsed());

    let now = std::time::Instant::now();
    e2();
    println!("time:{:?}", now.elapsed());

    let now = std::time::Instant::now();
    e3();
    println!("time:{:?}", now.elapsed());
}
