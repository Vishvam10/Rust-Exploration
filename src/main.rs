use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("\nStarting the app\n");

    let secret = rand::thread_rng().gen_range(1..=100);

    loop {
        let mut inp: String = String::new();
        io::stdin().read_line(&mut inp).expect("Read line failed");

        let inp: u32 = match inp.trim().parse() {
            Ok(res) => res,
            Err(_) => continue,
        };

        println!("I got : {}", inp);

        match inp.cmp(&secret) {
            Ordering::Less => println!("Too small"),
            Ordering::Equal => {
                println!("CORRECT !");
                break;
            }
            Ordering::Greater => println!("Too big"),
        }
    }
}
