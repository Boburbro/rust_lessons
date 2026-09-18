fn get_user_name() -> Result<String, ()> {
    Ok("John".to_string())
    // Err(())
}

fn get_first_name() -> Result<String, ()> {
    Ok("John".to_string())
}

fn get_last_name() -> Result<String, ()> {
    Ok("Doe".to_string())
}

fn get_full_name() -> Result<String, ()> {
    let first_name = get_first_name()?;
    let last_name = get_last_name()?;

    Ok(format!("{}, {}", first_name, last_name))
}

fn main() {
    println!("Hello, Error Handling");

    let value: Result<&str, Box<dyn std::error::Error>> = Ok("Hello");

    match value {
        Ok(value) => println!("{}", value),
        Err(error) => println!("{}", error),
    }

    // void result ----------------------------------------

    let value1: Result<&str, ()> = Ok("Hello");

    match value1 {
        Ok(value) => println!("{}", value),
        Err(_) => println!("Some error occurred"),
    }

    // expecting a value from result

    // let value2: Result<&str, ()> = Err(());
    // let unwrapped = value2.expect("I was expection a user");

    let username = get_user_name().expect("fail to get user name");
    println!("Hello {}!", username);

    let is_ok = get_user_name().is_ok();
    let is_err = get_user_name().is_err();

    println!("{}, {}", is_ok, is_err);

    let full_name = get_full_name();

    // match full_name {
    //     Ok(value) => println!("{}", value),
    //     Err(_) => println!("ERROR!"),
    // }

    let length = full_name.map(|s| s.len()).unwrap_or_default();
    println!("{}", length);
}
