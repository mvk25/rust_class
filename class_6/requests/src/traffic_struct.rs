enum TrafficLight {
    Green,
    Red,
    Light,
}

pub struct Traffic {
    state: TrafficLight,
}

impl Traffic {
    fn new() -> Self {
        Traffic {
            state: TrafficLight::Red
        }
    }

    fn go(&mut self) {
        self.state = TrafficLight::Green;
        println!("Gooooo!");
    }

    fn get_ready(&mut self) {
        self.state = TrafficLight::Orange;
    }

    fn stop(&mut self) {
        self.state = TrafficLight::Red;
    }
}

fn main() {
    let car = Traffic::new();
    car.go();
    car.get_ready();
    car.stop();

}