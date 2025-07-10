fn main() {
   //stored in heap
    let s1 = String::from("hello");
    // We are "moving" the ownership of the string data from s1 to s2,not moving the string.
    let s2 = s1;

    // Now s2 is the owner. s1 is no longer valid and cannot be used.

    println!("s2 has the value: {}", s2);

}