// No longer needed since rust 2018.
// extern crate rand;
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let b: bool = rng.gen();
    println!("Random bool = {}", b);
}
