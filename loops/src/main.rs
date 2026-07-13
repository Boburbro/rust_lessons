fn main() {
    let mut n = 0;
    loop {
        n += 1;

        if n == 5 {
            continue;
        } else if n > 10 {
            break;
        }

        println!("The value of n is {}", n)
    }

    let mut i = 1;

    while i <= 10 {
        i += 1;
        println!("The value of i is {}", i);
    }

    for j in 1..10 {
        println!("The number is {}", j);
    }

    let animals = vec!["Rabbit", "Dog", "Cat"];

    for (index, animal) in animals.iter().enumerate(){
        println!("The index is {}, the animal name is {}", index, animal)
    } 
}
