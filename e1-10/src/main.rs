fn e1() {}

fn main() {
    let now = std::time::Instant::now();
    e1();
    println!("time:{:?}", now.elapsed());
}
