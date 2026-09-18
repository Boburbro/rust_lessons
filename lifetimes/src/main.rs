fn get_full_name() -> &'static str {
    // lives for entire app
    "John Doe"
}

fn get_random_name<'l>(a: &'l str, _b: &'l str) -> &'l str {
    // Generic lifetime annotation
    a
}

struct Person<'a> {
    name: &'a str,
    surname: &'a str,
}

impl<'a> Person<'a> {
    fn say_hello(&self) {
        println!("Hello {} {}!", self.name, self.surname)
    }

    fn first_char_of_first_name(&self) -> &str {
        &self.name[0..1]
    }
}

enum Animal<'a> {
    Dog { name: &'a str },
}

fn main() {
    println!("Hello, Lifetimes!");

    let full_name = get_full_name();
    println!("Hello, {}!", full_name);

    let random_name = get_random_name("John", "Doe");
    println!("Hello, {}!", random_name);

    let person: Person = Person {
        name: "Bobur",
        surname: "Otaboyev",
    };

    person.say_hello();

    println!("{}", person.first_char_of_first_name());
}
