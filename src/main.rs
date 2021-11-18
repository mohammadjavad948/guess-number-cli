use rand::{thread_rng, Rng};

fn main() {

    loop {
        let x: u8 = gen_random();
        println!("{}", x);
    }
}

fn gen_random() -> u8{
    let mut rng = thread_rng();

    rng.gen_range(0..100)
}
