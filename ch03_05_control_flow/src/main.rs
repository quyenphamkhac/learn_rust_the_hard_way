fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let number = if true { 5 } else { 6 };
    println!("The value of number is: {number}");

    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {result}");

    let mut count = 0;
    'counting_up: loop {
        println!("outer count = {count}");
        let mut remaining = 10;

        loop {
            println!("inner remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 3 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");

    let mut number = 3;
    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
    println!("LIFTOFF!!!");

    let a = [10; 5];
    for element in a {
        println!("the value is: {element}");
    }

    for number in (1..4).rev() {
        println!("{number}!");
    }

    let f16 = fibinacci(16);
    println!("f16 = {f16}");
}

fn fibinacci(n: i32) -> u64 {
    let mut sum = 0;
    let mut last = 0;
    let mut curr = 1;
    if n < 0 {
        panic!("{} is negative!", n);
    } else if n == 0 {
        panic!("zero is not a right argument!");
    } else if n == 1 {
        return 1;
    } else {
        for _i in 1..n {
            sum = last + curr;
            last = curr;
            curr = sum;
        }
    }
    sum
}
