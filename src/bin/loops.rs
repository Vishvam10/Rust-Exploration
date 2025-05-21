use std::time::Duration;

fn main() {
    let topic: String = String::from("Loops Explorations");
    let stars: String = "*".repeat(topic.len());

    println!("\n{}", stars);
    println!("{}", topic);
    println!("{}\n", stars);

    exp3();
}

fn exp1() {
    let mut count: u32 = 0;

    let last_val: u32 = loop {
        count += 1;
        if count == 10 {
            break 100;
        }
    };

    println!("count : {}", count);
    println!("last val : {}", last_val);
}

fn exp2() {
    use std::time::Instant;

    let start: Instant = Instant::now();

    let mut count: u128 = 0;

    let last_val: u32 = loop {
        count += 1;
        if count == 1_000_000_000 {
            break 100;
        }
    };

    let duration: Duration = start.elapsed();

    println!("count : {}", count);
    println!("last val, duration : {}, {:?}", last_val, duration);
}

fn exp3() {
    use std::time::Instant;
    use std::hint::black_box;

    let start: Instant = Instant::now();

    let mut sum: u64 = 0u64;
    for i in 0..1_000_000_000 {
        sum = sum.wrapping_add(black_box(i));
    }

    let duration : Duration = start.elapsed();

    println!("sum : {}", sum);
    println!("duration : {:?}", duration)
}
