use std::collections::{HashMap, hash_map};

struct Point(f32, f32);

fn get_values() -> (String, String, i32) {
    ("Hello".to_string(), "World".to_string(), 20)
}

fn main() {
    let point = Point(12.5, 11.1);

    println!("point at ({}, {})", point.0, point.1);

    let values = ("Hello", "World", 30);

    println!("value is ({}, {}, {})", values.0, values.1, values.2);

    let hello = values.0;
    let world = values.1;
    let age = values.2;

    println!("{}, {}, {}", hello, world, age);

    let (hello1, _, _) = get_values();
    println!("hello1 is {}", hello1);

    //------------------------------------------------------------------------------

    let arr: [&str; 2] = ["foo", "bar"];

    for val in arr.iter() {
        println!("{}", val);
    }

    println!("{}", arr.len());

    let foo = arr[0];

    println!("{}", foo);

    let int_arr: [i32; 2] = [10, 20];

    let doubled = int_arr.iter().map(|x| x * 2);

    for i in doubled {
        println!("{}", i);
    }

    println!("----------------------------------------------");

    let mut vec_item: Vec<i32> = vec![1, 2, 3];

    vec_item.push(4);

    println!("val_item is {:?}", vec_item);

    vec_item.pop();
    vec_item.pop();

    println!("val_item is {:?}", vec_item);

    // vec_item.clear();

    vec_item.extend_from_slice(&[6, 7]);

    println!("val_item is {:?}", vec_item);

    let mut values1 = vec![1, 2, 3];
    let mut values2 = vec![4, 5, 6];
    println!("valuse1 = {:?}", values1);
    println!("valuse2 = {:?}", values2);
    values1.append(&mut values2);
    println!("valuse1 = {:?}", values1);
    if values1.contains(&3) {
        println!("yes");
    } else {
        println!("no")
    }

    values1.clear();

    if values1.is_empty() {
        println!("empty")
    } else {
        println!("not empty")
    }
    println!("----------------------------------------------");
    println!("----------------------------------------------");

    // ----------------------------------------------------
    // #### HASH MAP
    // ----------------------------------------------------

    let mut hash_values: HashMap<&str, &str> = HashMap::new();
    hash_values.insert("foo", "lol");
    hash_values.insert("bar", "baz");

    println!("{:?}", hash_values);

    hash_values.remove("foo");

    println!("{:?}", hash_values);

    if hash_values.contains_key("name") {
        println!("name exists {}", hash_values["name"]);
    } else {
        println!("No name");
    }

    let bar = hash_values["bar"];
    println!("bar is {}", bar);

    match hash_values.get("bar") {
        Some(value) => println!("match is {}", value),
        None => println!("Not found"),
    }

    hash_values.insert("nah", "yeah");
    hash_values.insert("brah", "bro");

    for (&k, &v) in &hash_values  {
        println!("{}, {}", k, v);
    }
}
