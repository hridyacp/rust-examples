fn main() {
    //considerring 10th Fibonacci number
    let n = 10;
    
    let result = fibonacci(n);

    println!("The Fibonacci number at position {} is {}", n, result);

    //another
    let n_2 = 20;
    println!("The Fibonacci number at position {} is {}", n_2, fibonacci(n_2));
}

// calculate the nth Fibonacci number.
fn fibonacci(n: u32) -> u64 {
    // The Fibonacci sequence starts with 0 and 1, using these two
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }

    let mut a = 0;
    let mut b = 1;

    //loop from 2 to n
    for _ in 2..=n {
        let next = a + b;
        
        a = b;     
        b = next;  
    }
    //returm b as it has the largest value in sequence
    b
}