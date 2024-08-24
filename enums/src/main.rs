use std::{thread, time};

enum TrafficLight {
    Red,
    Amber,
    Green
}

impl TrafficLight {
    fn show_message(&self) {
        match self {
            TrafficLight::Red => println!("Stop"),
            TrafficLight::Amber => println!("Stop, unless it's not safe to do so"),
            TrafficLight::Green => println!("Go, if the way is clear"),
        }
    }

    fn duration(&self) {
        let dur = match self {
            TrafficLight::Red => 5000,
            TrafficLight::Amber => 4000,
            TrafficLight::Green => 2000,
        };
        // Improvement for scaling - consider non-blocking or asynchronous approaches
        let millis = time::Duration::from_millis(dur);
        thread::sleep(millis);
    }

    fn next(&self) -> TrafficLight {
        match self {
            TrafficLight::Red => TrafficLight::Amber,
            TrafficLight::Amber => TrafficLight::Green,
            TrafficLight::Green => TrafficLight::Red,
        }
    }
}

fn main() {
    let mut colour = TrafficLight::Red;
    
    for _ in 0..10 {
        colour.show_message();
        colour.duration();
        // Update state of the light with each transition
        colour = colour.next();
    }
}
