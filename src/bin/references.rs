fn main() {
    let topic: String = String::from("References and Borrowing Explorations");
    let stars: String = "*".repeat(topic.len());

    println!("\n{}", stars);
    println!("{}", topic);
    println!("{}\n", stars);

    exp1();
}


fn exp1() {

    
    // pub fn push_str(&mut self, string: &str) {
    //  self.vec.extend_from_slice(string.as_bytes())
    // }
        
    // (1) push_str requires a mutable self
    // (2) push_str takes an immutable borrow (so, no &mut xyz)

    let s1: String = String::from("hello");

    println!("s1 : {}", s1);

    let mut s2 : String = String::from(s1);

    // Doesn't compile due to error : value borrowed here after move
    // println!("s1 : {}", s1);

    // Doesn't allow you to complile due to (1)
    // fn modify_1(s : &String) {
    //     s.push_str("something");
    // }

    // Doesn't allow you to complile due to (2)
    // fn modify_mut_1(s : &mut String) {
    //     {
    //         s.push_str(s);
    //     }
    // }

    fn modify_mut_2(s : &mut String) {
        s.push_str("something");
    }

    // Doesn't compile due to error : cannot mutate immutable variable `s1`
    // modify_mut_2(&mut s1);
    println!("s1, s2 : {}", s2);

    modify_mut_2(&mut s2);
    println!("s1, s2 : {}", s2);

}