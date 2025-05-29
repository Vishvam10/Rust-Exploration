fn main() {
    let topic: String = String::from("Struct Explorations");
    let stars: String = "*".repeat(topic.len());

    println!("\n{}", stars);
    println!("{}", topic);
    println!("{}\n", stars);

    exp3();
}

// Normal sturcts
fn exp1() {
    // #[derive(Debug)] allows you to print stuff using {:#?} or {:#?} instead
    // of doing this (or something better for pretty print) :

    // impl std::fmt::Display for Player {
    //     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    //         write!(f, "{} {}, {}, {}", self.name, self.health)
    //     }
    // }

    #[derive(Debug, Clone)]
    enum Abilities {
        Heal,
        Regenerate,
        Poision,
        Teleport,
    }

    #[derive(Debug, Clone)]
    struct Position {
        x: i64,
        y: i64,
        z: i64,
    }

    #[derive(Debug, Clone)]
    struct Player {
        name: String,
        health: u32,
        abilities: Vec<Abilities>,
        location: Position,
    }

    let p1: Player = Player {
        name: String::from("Player 1"),
        health: 100,
        abilities: vec![Abilities::Heal, Abilities::Poision],
        location: Position {
            x: 10,
            y: 10,
            z: 10,
        },
    };

    println!("Player 1 : {:#?}", p1);

    let p2: Player = Player {
        name: String::from("Player 2"),
        location: Position {
            x: 10,
            y: 10,
            z: 10,
        },
        ..p1
    };

    // IMPORTANT : Doesn't compile due to error : value borrowed here after
    // partial move

    // println!("Player 1 : {:#?}", p1);
    println!("Player 2 : {:#?}", p2);

    let p3: Player = Player {
        name: String::from("Player 3"),
        location: Position {
            x: 10,
            y: 10,
            z: 10,
        },
        ..p2.clone()
    };

    // This is fine as p2 is cloned
    println!("Player 2 : {:#?}", p2);
    println!("Player 3 : {:#?}", p3);

    // Mutability

    // Using "mut" makes ALL fields mutable. Rust doesn’t allow us to mark
    // only certain fields as mutable

    let mut p4 = Player {
        name: String::from("Player 4"),
        health: 100,
        abilities: vec![Abilities::Heal],
        location: Position {
            x: 10,
            y: 10,
            z: 10,
        },
    };

    p4.abilities.push(Abilities::Regenerate);
    p4.name = String::from("Player 4 Updated");

    println!("Player 4 : {:#?}", p4);
}

// Tuple structs
fn exp2() {
    #[derive(Debug)]
    struct Vec3D(i64, i64, i64);

    let origin: Vec3D = Vec3D(0, 0, 0);
    let pc: Vec3D = Vec3D(10, 13, 239);

    println!("origin : {:#?}", origin);
    println!("\npc : {:#?}", pc);
    println!("-- pc x : {:#?}", pc.0);
    println!("-- pc y : {:#?}", pc.1);
    println!("-- pc z : {:#?}", pc.2);

    #[derive(Debug)]
    struct EigenBasis(Vec3D, Vec3D, Vec3D);

    let basis: EigenBasis = EigenBasis(origin, Vec3D(10, 10, 10), Vec3D(20, 20, 20));

    println!("\n\n");
    println!("eigen basis : {:#?}", basis);
    println!("-- eigen basis p1 : {:#?}", basis.0);
    println!("-- eigen basis p2 : {:#?}", basis.1);
    println!("---- eigen basis p2.x : {:#?}", basis.1.0);
    println!("---- eigen basis p2.y : {:#?}", basis.1.0);
    println!("---- eigen basis p2.z : {:#?}", basis.1.0);
    println!("-- eigen basis p3 : {:#?}", basis.2);

    // Destructuring stuff : need to name the type of the struct

    // Does not compile due to error : use of moved value: origin
    // let Vec3D(x, y, z) = origin;

    let coords = Vec3D(1, 1, 1);
    let Vec3D(x, y, z) = coords;

    println!("x, y, z : {}, {}, {}", x, y, z);
}

// Constructors and whatnot

fn exp3() {
    #[derive(Debug)]
    struct Vec2D(i64, i64);

    #[derive(Debug)]
    struct Polygon {
        vertices: Vec<Vec2D>,
    }

    impl Polygon {
        fn triangle(p1: Vec2D, p2: Vec2D, p3: Vec2D) -> Self {
            Self {
                vertices: vec![p1, p2, p3],
            }
        }

        fn square(p1: Vec2D, p2: Vec2D, p3: Vec2D, p4: Vec2D) -> Self {
            Self {
                vertices: vec![p1, p2, p3, p4],
            }
        }

        fn area(&self) -> i64 {
            let n = self.vertices.len();
            let mut area = 0;

            for i in 0..n {
                let Vec2D(x1, y1) = self.vertices[i];
                let Vec2D(x2, y2) = self.vertices[(i + 1) % n];
                area += (y1 + y2) * (x2 - x1);
            }

            area.abs() / 2
        }
    }

    let tr = Polygon::triangle(Vec2D(0, 0), Vec2D(10, 0), Vec2D(10, 10));

    let tr_area = tr.area();
    println!("Triangle : {:#?}", tr);
    println!("Triangle Area : {:#?}", tr_area);

    let sq = Polygon::square(Vec2D(0, 0), Vec2D(10, 0), Vec2D(10, 10), Vec2D(0, 10));

    let sq_area = sq.area();

    println!("Square : {:#?}", sq);
    println!("Square Area : {:#?}", sq_area);
}
