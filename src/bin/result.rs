fn parse_number(s: &str) -> Result<i32, std::num::ParseIntError> {
   
    s.parse::<i32>()
}

fn main() {
    let good_input = "42";
    let bad_input = "hello";

    let result1 = parse_number(good_input);
    match result1 {
        Ok(n) => println!("Successfully parsed number: {}", n),
        Err(e) => println!("Failed to parse: {}", e),
    }

    let result2 = parse_number(bad_input);
    match result2 {
        Ok(n) => println!("Successfully parsed number: {}", n),
        Err(e) => println!("Failed to parse: {}", e),
    }
}