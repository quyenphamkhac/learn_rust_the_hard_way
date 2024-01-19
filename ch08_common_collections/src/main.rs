use std::collections::HashMap;
fn main() {
    let v = vec![1, 2, 3, 4, 5];
    let mut v1 = Vec::new();
    v1.push(5);
    v1.push(6);
    v1.push(7);
    v1.push(8);

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(t) => println!("The third element is {t}"),
        None => println!("There is no third element."),
    }

    let first = &v1[0];
    println!("The first element is: {first}");
    v1.push(9);

    for i in &v1 {
        println!("{i}");
    }

    for i in &mut v1 {
        *i += 50;
    }

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let _row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    let mut _s = String::new();
    let data = "initial contents";
    let s = data.to_string();
    println!("{s}");
    let s = "initial contents".to_string();
    println!("{s}");
    let s = String::from("initial contents");
    println!("{s}");
    let mut hello = String::from("hello");
    hello.push_str(", world!");
    println!("{hello}");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{s1}-{s2}-{s3}");
    println!("{s}");

    let hello = "Здравствуйте";

    let s = &hello[0..4];
    println!("{s}");
    for c in hello.chars() {
        println!("{c}");
    }

    for b in hello.bytes() {
        println!("{b}");
    }

    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("Team {team_name} has {score} point(s).");

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    scores.insert(String::from("Blue"), 11);
    scores.insert(String::from("Blue"), 25);
    println!("{:?}", scores);

    scores.entry(String::from("Yellow")).or_insert(100);
    scores.entry(String::from("Red")).or_insert(1000);
    println!("{:?}", scores);

    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}
