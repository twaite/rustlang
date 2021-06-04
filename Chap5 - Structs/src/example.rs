struct User {
    username: String,
    email: String,
    sign_in_count: u64, // that's a lot of attempts!
    active: bool,
}

fn example() {
    // Structs
    println!("--- Structs ---");

    // Now we can create an instance of the struct:
    let mut user1 = User {
        email: String::from("test@test.com"),
        username: String::from("legolas1297"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("new_email@test.com");

    let user2 = create_user(String::from("another@test.com"), String::from("gimli0582"));

    // Sort of like spread, but it only fills undefined values, it doesn't overwrite
    let user3 = User {
        email: String::from("third@test.com"),
        username: String::from("aragorn87912"),
        ..user2
    };

    // Can also name tuples:
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

fn create_user(email: String, username: String) -> User {
    // Can use shorthand to generate structs, nice.
    User {
        username,
        email,
        active: true,
        sign_in_count: 1,
    }
}
