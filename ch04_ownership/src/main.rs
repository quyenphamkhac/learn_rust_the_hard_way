fn main() {
    let s = String::from("hello");
    takes_ownership(s);
    let x = 5;
    makes_copy(x);

    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s2 = takes_and_gives_back(s2);
    println!("{} {}", s1, s2);

    let s3 = String::from("hello");
    let (s3, len) = calculate_length(s3);
    println!("The length of '{}' is {}", s3, len);

    let len = calculate_length_v2(&s3);
    println!("The length of '{}' is {}", s3, len);
}

fn calculate_length_v2(s: &String) -> usize {
    s.len()
}

fn calculate_length(s: String) -> (String, usize) {
    let len = s.len();
    (s, len)
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}
