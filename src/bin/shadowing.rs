fn main() {

    let topic : String = String::from("Slices Explorations");
    let stars :  String = "*".repeat(topic.len());

    println!("\n{}", stars);
    println!("{}", topic);
    println!("{}\n", stars);

    exp2();
}

fn exp1() {

    let mut x : u32 = 10;
    let mut y : u32 = 20;

    println!("x : {}", x);
    println!("y : {}", y);

    {
        x = x + 1;
        y = y + 1;
        println!("-- x : {}", x);
        println!("-- y : {}", y);

        {
            x = x + 1;
            y = y + 1;
            
            println!("---- x : {}", x);
            println!("---- y : {}", y);
        }

        println!("-- x : {}", x);
        println!("-- y : {}", y);

    }

    println!("x : {}", x);
    println!("y : {}", y);

}

fn exp2() {

    let mut x : u32 = 10;

    println!("x : {}", x);
    
    {
        let mut x : u32 = 15;
        println!("-- x : {}", x);
        
        x += 1;
        
        println!("-- x : {}", x);
    }
    
    println!("x : {}", x);
}
