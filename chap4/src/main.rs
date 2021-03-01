fn main() {
    let mut s = String::from("hello");

    s.push_str(", world!");

    println!("{}", s);

    // Scope stuff
    let s1 = String::from("hello");
    let s2 = s1;

    // throws an error
    // println!("{}, world!", s1);
    println!("{}, world!", s2);

    // The above example doesn't work because
    // the amount of space that needs to be allocated
    // for the dynamic string is unknown
    // but the following will work because we can
    // safely guarantee the size of the variables:

    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);
    // To assign like this you have to use a type with
    // the Copy method implemented. You can also use
    // Tuples that have only variables with the Copy
    // method implemented

    let s2 = String::from("hello");

    takes_ownership(s2);

    // This will error because it's no longer in the scope, mem is freed
    // println!("{}", s2);

    let z = 5;
    makes_copy(z);

    println!("Some int: {}", z);

    let some_string = String::from("hello");

    let s3 = takes_and_gives_back(some_string);

    println!("Still in scope? {}", s3);

    let pass_by_ref = String::from("test");

    by_ref(&pass_by_ref);

    println!("Can I use this again? {}", pass_by_ref);

    let s4 = String::from("hello");

    let (s5, len) = calculate_length(s4);

    // s4 is no longer valid here
    println!("The length of '{}' is {}.", s5, len);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string); // variables is droped after this because we don't use it again
}

fn makes_copy(some_int: i32) {
    println!("{}", some_int);
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn by_ref(test: &String) {
    println!("By ref: {}", test);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}
