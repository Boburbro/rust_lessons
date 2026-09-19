use std::fmt;

struct Person {
    first_name: String,
    last_name: String,
    age: u8,
}

trait HasFullName {
    fn full_name(&self) -> String;
}

impl HasFullName for Person {
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
}

trait CanInitializeWithFullName {
    fn new(full_name: &str) -> Self;
}

trait CanRun {
    fn run(&self);
}

impl CanRun for Person {
    fn run(&self) {
        // todo
    }
}

fn print_details<T: HasFullName + CanRun>(value: &T) {
    println!("{}", value.full_name());
    value.run();
}

impl CanInitializeWithFullName for Person {
    fn new(full_name: &str) -> Self {
        let parts: Vec<&str> = full_name.split(" ").collect();
        Person {
            first_name: parts[0].to_string(),
            last_name: parts[0].to_string(),
            age: 0,
        }
    }
}

impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} {} is {} years old",
            self.first_name, self.last_name, self.age
        )
    }
}

fn print_full_name_and_age(value: &impl HasFullName) {
    println!("{}", value.full_name())
}

fn main() {
    println!("Hello, Traits!");

    let person = Person {
        first_name: "John".to_string(),
        last_name: "Doe".to_string(),
        age: 30,
    };

    println!("{}", person.age);

    println!("{}", person.full_name());

    let person1 = Person::new("Bob Otaboy");

    println!("{}", person1);

    let person2 = Person::new("A aa");
    print_full_name_and_age(&person2);

    print_details(&person);
}
