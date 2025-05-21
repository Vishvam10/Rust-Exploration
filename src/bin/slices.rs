fn main() {
    let topic: String = String::from("Shadowing Explorations");
    let stars: String = "*".repeat(topic.len());

    println!("\n{}", stars);
    println!("{}", topic);
    println!("{}\n", stars);

    exp3();
}

fn exp1() {
    let s: String = String::from("Hello");

    // Doesn't compile due to error : a doesn't have a size known at
    // compile-time

    // let a: str = s[0..2];

    // println!("a : {}", a);

    let a = &s[0..2];
    let b = &s[0..4];

    // for (i, ch) in a.chars().enumerate() {
    //    print!("i, char : {}, {}\n", i, ch);
    // }

    let mut c = &s[0..1];
    c = "a";

    println!("a, b, c, s : {}, {}, {}, {}\n", a, b, c, s);
}

fn exp2() {
    let mut s: String = String::from("Hello");
    let mut a = &s[0..1];

    // At this stage, "something" is assign to a and a loses the reference to
    // &s[0..1]
    a = "something";

    println!("a, s : {}, {}\n", a, s);
}

fn exp3() {

    fn a(s: &mut String) {
        s.to_uppercase();
    }

    fn b(s: &mut String) {
        let res: String = s.to_uppercase();
        // Doesn't compile due to error : expected &mut String, found String
        // s = res;

        // So, we need to get the data from the mutable reference using the
        // derefence operator

        *s = res;
    }

    let mut s: String = String::from("hello");

    println!("s : {}", s);

    a(&mut s);
    println!("a(s) : {}", s);
    
    b(&mut s);
    println!("b(s) : {}", s);
}
