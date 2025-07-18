fn main() {
  let squared_sum = |x: i32, y: i32| {
    
        let mut sum: i32 = x + y;
        let mut result: i32 = sum * sum;
        
        return result;
    };
    
    // call the closure
    let result = squared_sum(5, 3);
    
    println!("Result = {}", result);
}