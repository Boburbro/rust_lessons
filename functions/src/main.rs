fn say_hello_world(message: &str) {
    println!("{}", message);
}

fn get_hello_world() -> String {
    String::from("Hello World!")
}

fn say_hello_to_person(person_name: &str) {
    let message = get_hello_to_person(person_name);
    println!("{}", message);
}

fn get_hello_to_person(person_name: &str) -> String {
    format!("Hello {}", person_name)
}

fn main() {
    let message = get_hello_world();
    say_hello_world(&message);
    say_hello_world(&message);

    say_hello_to_person("Bobur");
    say_hello_to_person("Ali");

    // ---------------------------------------------

    let say_hello_line = |name: &str| format!("Hello {}!", name);

    println!("{}", say_hello_line("Bobur"));

    // ---------------------------------------------

    let say_hello_with_fish =
        |first_name: &str, last_name: &str| format!("Hello {} {}", first_name, last_name);

    println!("{}", say_hello_with_fish("Bobur", "Otaboyev"));

    let multiply_by_2 = |x: i32| x * 2;

    println!("{}", multiply_by_2(21));

    let ask_for_age = || {
        // not any input
        // empty function
        10
    };

    println!("age: {}", ask_for_age());

    let multp_2 = |x: i32| x * 2;
    let mpl = multp_2;

    println!("multp_2: {}", multp_2(2));
    println!("mpl: {}", mpl(2));

    fn prosess_name(name: &str, callback: fn(&str) -> ()) {
        callback(name);
    }

    prosess_name("Bobur", |name: &str| {
        println!("{} is on callback", name);
    });

}
