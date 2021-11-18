use rand::Rng;
use rand::thread_rng;

fn main() {

    let mut rng = thread_rng();

    let x: u8 = rng.gen_range(0..100);
    println!("{}", x);
}
