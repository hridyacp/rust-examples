fn main() {

    let fahrenheit_temp = 68.0;

  //conversion
    let celsius_temp = fahrenheit_to_celsius(fahrenheit_temp);
    println!("{}°F is {:.1}°C\n", fahrenheit_temp, celsius_temp);


    //list of teperature conversion
    println!("Batch Fahrenheit to Celsius");

    let fahrenheit_values = [32.0, 50.0, 77.0, 104.0, 212.0];

    for temp in fahrenheit_values {
        let converted_celsius = fahrenheit_to_celsius(temp);
        println!("{}°F is {:.1}°C", temp, converted_celsius);
        if temp == 32.0 {
            println!("(This is the freezing point of water!)");
        }
    }
    println!(); 

    println!("Batch Celsius to Fahrenheit");
    let celsius_values = [0.0, 10.0, 25.0, 40.0, 100.0];
    let mut i = 0;
    while i < celsius_values.len() {
        let temp = celsius_values[i];
        let converted_fahrenheit = celsius_to_fahrenheit(temp);
        println!("{}°C is {:.1}°F", temp, converted_fahrenheit);

        if temp == 100.0 {
            println!("(This is the boiling point of water!)");
        } else if temp == 0.0 {
            println!("(This is the freezing point of water!)");
        }

        i = i + 1;
    }
}


//convert Fahrenheit to Celsius.
fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * (5.0 / 9.0)
}

//convert Celsius to Fahrenheit.
fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * (9.0 / 5.0)) + 32.0
}