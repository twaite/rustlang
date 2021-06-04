// Declaring consts, note that you can use _ for commas in numbers:
// const MAX_POINTS: u32 = 100_000;

pub fn main() {
    println!("VARIABLES:\n--");

    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    let y = 5;
    let y = y + 1;
    let y = y * 2;

    println!("The value of y is: {}", y);

    let spaces = "   ";
    let spaces = spaces.len();

    println!("The value of spaces is: {}\n", spaces);

    // This errors because you can't change the type of a mutable variable:
    // let mut spaces = "   ";
    // let spaces = spaces.len();
}
