enum TrafficLight {
    Red,
    Yellow,
    Green,
}

//add method to enum
impl TrafficLight {
   ///using match
    fn time_to_wait(&self) -> u8 {
        match self {
            TrafficLight::Red => 60,    
            TrafficLight::Yellow => 3,   
            TrafficLight::Green => 0,   
        }
    }
}

fn main() {
    let current_light = TrafficLight::Red;
    println!("Current light is Red. You must wait {} seconds.", current_light.time_to_wait());

    let next_light = TrafficLight::Green;
    println!("Next light is Green. You must wait {} seconds.", next_light.time_to_wait());
}