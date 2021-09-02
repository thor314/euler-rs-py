fn e1() {
    let threes = 3 * 333 * 334 / 2;
    let fives = 5 * 199 * 200 / 2;
    let fifteens = 15 * 66 * 67 / 2;
    let out = threes + fives - fifteens;
    println!("{}", out);
}

fn main() {
    let now = std::time::Instant::now();
    e1();
    println!("time:{:?}", now.elapsed());
}
