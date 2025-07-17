fn find_first_even_number(numbers: &[i32]) -> Option<i32> {
    for &num in numbers {
        if num % 2 == 0 {
            return Some(num); 
        }
    }
    None
}

fn main() {
    let my_numbers = vec![1, 3, 5, 8, 9];
    let first_even = find_first_even_number(&my_numbers);

    match first_even {
        Some(n) => println!("The first even number is: {}", n),
        None => println!("No even numbers were found."),
    }
}