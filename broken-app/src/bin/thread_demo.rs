use broken_app::concurrency;

fn main() {
    let total = concurrency::race_increment(10_000, 4);
    println!("race_increment total: {total}");
    println!("counter after read: {}", concurrency::read_after_sleep());
}
