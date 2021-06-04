pub fn main() {
    println!("DATA TYPES:\n--");

    let uint: u32 = 241;
    let int: i32 = -234;

    println!("uint: {}, int: {}", uint, int);

    let float = 2.0; // f64
    let float32: f32 = 3.0; // f32

    println!("float64: {}, float32: {}", float, float32);

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;

    // remainder
    let remainder = 43 % 5;

    println!(
        "sum: {}, diff: {}, product: {}, quotient: {}, remainder: {}",
        sum, difference, product, quotient, remainder
    );

    let t = true;
    let f: bool = false;

    println!("t: {}, f: {}", t, f);

    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    println!("Chars: {} {} {}\n", c, z, heart_eyed_cat);

    println!("Compound types:");

    let tup: (i32, f64, u8) = (500, 5.4, 1); // don't know how to log this yet

    let (x, y, z) = tup; // interesting, like JS destructuring

    println!("Destructured tup: {} {} {}", x, y, z);

    // Tuples can also be assigned by index:
    println!("By index: {} {} {}", tup.0, tup.1, tup.2);

    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    println!("Third month: {}", months[2]);

    // Filling an array with the same value:
    let a = [3; 5];

    println!("Filled array with: {}\n", a[4]);
}
