fn main() {
    let number_list = vec![123, 43, 58, 84, 90];
    let largest = find_largest_number(&number_list).unwrap();
    println!("{}", largest);
    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let result = find_largest_number(&number_list).unwrap();
    println!("The largest number is {}", result);
}

fn find_largest_number(numbers: &Vec<i32>) -> Option<i32> {
    if numbers.is_empty() {
        return None;
    }

    let mut largest = numbers[0];
    for &num in numbers.iter() {
        if num > largest {
            largest = num;
        }
    }
    Some(largest)
}
