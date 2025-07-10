struct Rectangle {
    width: u32,
    height: u32,
}

//implementation to add methods to struct Rectangle
impl Rectangle {
    fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    // This is a "method" because its first parameter is &self, which represents the instance of the struct the method is being called on.
    fn area(&self) -> u32 {
        self.width * self.height
    }
}


fn main() {
   //new instance using new
    let rect1 = Rectangle::new(30, 50);

    //another method to create an instance with fields
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    //calling method 
    println!(
        "The area of rect1 ({}x{}) is {} square pixels.",
        rect1.width, rect1.height, rect1.area()
    );

    println!(
        "The area of rect2 ({}x{}) is {} square pixels.",
        rect2.width, rect2.height, rect2.area()
    );
}