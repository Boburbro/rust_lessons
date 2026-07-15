const MY_AGE: u8 = 20;

fn main() {
    // let mut x: i32 = 2147483647 ;

    // println!("{}", x);

    let x: i32 = 2147483647;
    let f: f32 = 6.7;
    let b: bool = false;

    println!("{}, {}, {}", x, f, b);

    let mut name = "John";

    name = "Jane";

    let data = "foo";
    let data = 20;

    println!("Your name is {}", name);

    const MY_AGE: u8 = 19;

    println!("{}", MY_AGE);

    let personal_data: (u8, &str) = (19u8, "John");

    println!("{}", personal_data.1);
}
