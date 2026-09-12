struct Person {
    name: String,
    age: u8,
}

fn say_hello(person: &Person) {
    println!(
        "Hello\nMy name is {}\nI am {} years old",
        person.name, person.age
    );
}

// ----------------------------------------------------

struct Point(f64, f64, f64);

impl Point {
    fn twice(&self) -> Point {
        Point(self.0 * 2.0, self.1 * 2.0, self.2 * 2.0)
    }

    fn make_twice(&mut self) {
        self.0 *= 2.0;
        self.1 *= 2.0;
        self.2 *= 2.0;
    }

    fn describe(&self) {
        println!("Point is at ({}, {}, {})", self.0, self.1, self.2);
    }
}

impl Point {
    fn self_print(&self) {
        println!("{} {} {}", self.0, self.1, self.2);
    }

    fn zero() -> Point {
        Point(0.0, 0.0, 0.0)
    }
}

fn main() {
    let person: Person = Person {
        name: "Bobur".to_string(),
        age: 20,
    };

    let person2: Person = Person {
        name: "Doe".to_string(),
        ..person
    };

    say_hello(&person);
    say_hello(&person2);

    // --------------------------------------------------

    let point: Point = Point(1.0, 2.0, 3.0);
    println!("x = {}, y = {}, z = {},", point.0, point.1, point.2);

    point.describe();
    point.self_print();
    let point2: Point = point.twice();
    point2.describe();

    let mut point3: Point = point2.twice();
    point3.describe();
    point3.make_twice();
    point3.describe();

    let point4: Point = Point::zero();

    point4.describe();
}
