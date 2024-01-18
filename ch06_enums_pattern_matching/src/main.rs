fn main() {
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    println!("{} {:?}", home.address, home.kind);
    println!("{} {:?}", loopback.address, loopback.kind);

    let home1 = IpAddrV2::V4(127, 0, 0, 1);
    let loopback1 = IpAddrV2::V6(String::from("::1"));

    println!("{:?}", home1);
    println!("{:?}", loopback1);

    let m = Message::Write(String::from("hello"));
    let m1 = Message::Quit;
    let m3 = Message::ChangeColor(255, 255, 255);
    m.call();
    m1.call();
    m3.call();

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("{:?} {:?} {:?}", five, six, none);

    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("{}", max),
        _ => (),
    }

    let coin = Coin::Quarter(UsState::Alaska);
    let mut count = 0;
    let cents = value_in_cents(&coin);
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
        count += cents;
    } else {
        count += 1;
    }
    println!("{}", count);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            match state {
                UsState::Alabama => 25,
                UsState::Alaska => 10,
            }
        }
        _ => 10,
    }
}

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
enum IpAddrV2 {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {}
}
