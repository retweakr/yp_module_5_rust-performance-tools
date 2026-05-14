use broken_app::{average_positive, normalize};

fn main() {
    let numbers = [-5, 5, 15];
    println!("average_positive: {}", average_positive(&numbers));
    println!("normalize: {}", normalize(" Hello\tWorld "));
}
