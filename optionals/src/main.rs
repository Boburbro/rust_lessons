fn main() {
    println!("Hello, Oprionals!");

    // let name: Option<&str> = None;
    let name: Option<&str> = Some("Bob");

    match name {
        Some(name) => println!("Hello, {}!", name),
        None => println!("There is no name"),
    }

    // let unwrapped_name = name.expect("name was not provided");
    let unwrapped_name = name.unwrap();

    println!("Name is {}", unwrapped_name);

    let mut age: Option<i8> = Some(20);
    match age.as_mut() {
        Some(age) => *age += 10,
        None => println!("No age"),
    }

    println!("----------------------------------------------");

    let new_name: Option<&str> = Some("Jane Doe");
    // let new_name: Option<&str> = None;

    let unwrapped_new_name = new_name.unwrap_or("Bob");

    println!("name is {}", unwrapped_new_name);
}
