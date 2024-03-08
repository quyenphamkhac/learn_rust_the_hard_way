use std::{
    fs::{read_to_string, File},
    io::{self, ErrorKind, Read},
};

fn main() {
    let filename = "hello1.txt";
    let text2 = read_file_v2(filename);
    println!("{}", text2);
    let text = read_file_v1(filename);
    println!("{}", text);
    let text3 = read_file_v3(filename);
    println!("{}", text3);
    let filename1 = "username.txt";
    let text4 = read_file_v4(filename1).unwrap();
    println!("{}", text4);
    let text5 = read_file_v5(filename1).unwrap();
    println!("{}", text5);
    let text6 = read_file_v6(filename1).unwrap();
    println!("{}", text6);
    let text7 = read_file_final(filename1).unwrap();
    println!("{}", text7);
}

// Recoverable Errors with Result
fn read_file_v1(filename: &str) -> String {
    let file_result = File::open(filename);
    let mut text_file = match file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
    let mut text = String::new();
    text_file
        .read_to_string(&mut text)
        .expect("Unable to read string");
    text
}

// Matching on Different Errors
fn read_file_v2(filename: &str) -> String {
    let file_result = File::open(filename);
    let mut text_file = match file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(filename) {
                Ok(created_file) => created_file,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };
    let mut text = String::new();
    text_file
        .read_to_string(&mut text)
        .expect("Unable to read string");
    text
}

// Unwrap and Expect
fn read_file_v3(filename: &str) -> String {
    let mut text_file = File::open(filename).unwrap();
    let mut text = String::new();
    text_file
        .read_to_string(&mut text)
        .expect("Unable to read string");
    text
}

// Propagating Errors
fn read_file_v4(filename: &str) -> Result<String, io::Error> {
    let file_result = File::open(filename);

    let mut text_file = match file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut text = String::new();

    match text_file.read_to_string(&mut text) {
        Ok(_) => Ok(text),
        Err(e) => Err(e),
    }
}

// Operator ?
fn read_file_v5(filename: &str) -> Result<String, io::Error> {
    let mut text_file = File::open(filename)?;
    let mut text = String::new();
    text_file.read_to_string(&mut text)?;
    Ok(text)
}

// Operator ? Chaining
fn read_file_v6(filename: &str) -> Result<String, io::Error> {
    let mut text = String::new();
    File::open(filename)?.read_to_string(&mut text)?;
    Ok(text)
}

fn read_file_final(filename: &str) -> Result<String, io::Error> {
    read_to_string(filename)
}
