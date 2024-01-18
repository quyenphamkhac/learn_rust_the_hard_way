fn main() {
    let user1 = User {
        active: true,
        username: String::from("someuser123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    let user2 = build_user(String::from("pkquyen"), String::from("pkquyen@example.com"));

    println!("user 1 is {:?}", user1);
    println!("{}", user1.active);

    println!("user 2 is {:?}", user2);
    println!("{} {} {}", user2.username, user2.email, user2.sign_in_count);

    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    let rect2 = Rectangle {
        width: 30 / scale,
        height: 25,
    };

    println!("{:?} {} {}", rect1, rect1.width, rect1.height);
    println!(
        "Width is greater than zero {}. Area is {}",
        rect1.width(),
        rect1.area()
    );
    println!("Rect1 can hold Rect2: {}", rect1.can_hold(&rect2));
    println!("Rect2 can hold Rect1: {}", rect2.can_hold(&rect1));
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn width(&self) -> bool {
        self.width > 0
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(username: String, email: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 0,
    }
}
