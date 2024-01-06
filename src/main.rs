struct User {
    name: String,
    email: String,
}

impl User {
    // group of functions for the struct
    fn new(name: &str) -> User {
        // name is a reference
        // refers to same memory address as caller var
        User {
            name: name.to_string(),
            email: format!("{}@example.com", name),
        }
    }

}


// Railway oriented programming (Guard Rails)
// metaphor of switching between success and failure tracks.

fn get_user_option(name: &str) -> Option<User> {
    if name == "Greg" {
        Some(User::new(name))
    } else {
        // None not a value part of Option type
        None  
    }
}

fn get_user_result(name: &str) -> Result<User, String> {
    // example of  monadic error handling
    if name == "Greg" {
        Ok(User::new(name))
    } else {
        Err(String::from("Not Found"))
    }
}

fn main() {
    let _res = exploring::add(2, 3);
    println!(
    // variadic macro ("!" defines a macro)
    "Hello, world!");

    let user = User::new("greg");
    println!("Hi, {}!", user.name);
    println!("email, {}!", user.email);
    // the following would error:
    // user.email = "bob@example.com";

    let mut user = User::new("greg");
    user.email = "bob@example.com".to_string();
    println!("email, {}!", user.email);

    // Option Vs Result which to use?
    // function that looks for item in dict (option)
    // ok state Vs err state of program (example file missing)

    let user_option = get_user_option("Greg");
    match user_option {
        Some(user) => println!("Hi, {}!", user.name),
        None => println!("Not Found")
    }

    let user_result = get_user_result("Bob");
    match user_result {
        Ok(user) => println!("Hi {}", user.name),
        Err(err) => println!("{}", err)
    }
    
}