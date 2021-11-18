mod util;

fn main() {

    loop {
        let x: u8 = util.gen_random();
        println!("{}", x);
    }
}

