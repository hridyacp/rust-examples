fn main() {
    let mut my_string = String::from("Rust is cool");
//immutable borrowing
    // my_string is still the owner. The function just borrows it for a moment.ass an immutable reference `&` to the function
    let length = calculate_length(&my_string);

    println!("The string '{}' has length {}.", my_string, length);
    // Since we still own my_string, we can continue to use it here. 


    //mutable Borrowing

    println!("\nOriginal string: {}", my_string);

    // To modify the string, we pass a mutable reference &mut. The variable my_string is also declared with `mut`.
    add_suffix(&mut my_string);

    println!("Modified string: {}", my_string);
}

// Function can reac the string, but cannot change it.
fn calculate_length(s: &String) -> usize {
    s.len()
}

//This function takes a MUTABLE reference to a String. It can reac and change the string.
fn add_suffix(s: &mut String) {
    s.push_str("!");
}