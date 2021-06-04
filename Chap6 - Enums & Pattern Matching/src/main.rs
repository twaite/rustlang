#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

#[derive(Debug)]
enum IpAddrSimple {
    V4(String),
    V6(String),
}

#[derive(Debug)]
enum IpAddrBest {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Colorado,
    Texas,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    // Enums

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    println!("Our IpAddrKinds are {:?} and {:?}\n", four, six);

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    println!("Networks are: {:#?} {:#?}\n", home, loopback);

    let home2 = IpAddrSimple::V4(String::from("127.0.0.1"));
    let loopback2 = IpAddrSimple::V6(String::from("::1"));

    println!("Simple enum: {:#?} {:#?}\n", home2, loopback2);

    let home3 = IpAddrBest::V4(127, 0, 0, 1);
    let loopback3 = IpAddrBest::V6(String::from("::1"));

    println!("Simple enum: {:#?} {:#?}\n", home3, loopback3);

    // Pattern Matching
    println!(
        "Some coins: {}c {}c\n",
        value_in_cents(Coin::Penny),
        value_in_cents(Coin::Quarter(UsState::Colorado))
    );

    // Matching w/ Option:
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("Some options: {:?} {:?} {:?}", five, six, none);

    // Should print 'three' once
    only_act_on_three(Some(5));
    only_act_on_three(Some(3));
    only_act_on_three(None);

    // Should print 'three (short)' once
    shorthand_act_on_three(Some(5));
    shorthand_act_on_three(Some(3));
    shorthand_act_on_three(None);

    // Allowing a default state with let then:
    let mut count = 0;
    let coin = Coin::Quarter(UsState::Texas);

    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }

    println!("count of non quarters: {}", count);
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn only_act_on_three(val: Option<u8>) {
    match val {
        Some(3) => println!("three"),
        _ => (),
    }
}

fn shorthand_act_on_three(val: Option<u8>) {
    if let Some(3) = val {
        println!("three (short)");
    }
}
