pub fn main() {
    println!("FUNCTIONS:\n--");

    another_function(5, 6);

    // You can create new scopes with curly braces
    // aka expressions
    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}, we didn't use x: {}", y, x);

    let x = five();
    println!("The value of x is: {}", x);

    let x = plus_one(5);
    println!("(plus_one) The value of x is: {}\n", x);
}

// functions can be defined in any order
fn another_function(x: i32, y: i32) {
    println!("Another function with vals: {}, {}", x, y);
}

// Arrow declares type, no return needed?
fn five() -> i32 {
    5
}

// apparently no semicolon indicates the return
fn plus_one(x: i32) -> i32 {
    x + 1
}
