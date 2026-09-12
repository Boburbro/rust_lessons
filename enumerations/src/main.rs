#[derive(PartialEq)]
enum AnimalType {
    Dog,
    Cat,
    Rabbit,
}

fn print_animal(animal: AnimalType) {
    match animal {
        AnimalType::Dog => println!("Woof!"),
        AnimalType::Cat => println!("Meow!"),
        AnimalType::Rabbit => println!("Hoot!"),
    }
}

enum Shapes {
    Circle { radius: f64, center: (f64, f64) },
    Rectangle { width: f64, height: f64 },
}

fn print_shape(shape: Shapes) {
    match shape {
        Shapes::Circle {
            radius: r,
            center: c,
        } => println!("The circle is {} and ({}, {})", r, c.0, c.1),
        Shapes::Rectangle {
            width: w,
            height: h,
        } => println!("The ractangle is {} and {}", w, h),
    }
}

fn main() {
    let animal: AnimalType = AnimalType::Cat;
    let animal1: AnimalType = AnimalType::Dog;
    let animal2: AnimalType = AnimalType::Rabbit;
    print_animal(animal);
    print_animal(animal1);
    print_animal(animal2);

    // =============================================

    let rectangle = Shapes::Rectangle {
        width: 22.4,
        height: 55.1,
    };
    let circle = Shapes::Circle {
        radius: 44.1,
        center: (12.2, 42.1),
    };

    print_shape(rectangle);
    print_shape(circle);
}

