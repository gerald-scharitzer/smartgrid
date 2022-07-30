struct Node {
	/// current power in watt - Producers have positive power. Consumers have negative power.
    power: f64,
	/// maximum power in watt - the peak capacity of producers
    power_max: f64,
	/// minimum power in watt - the peak capacity of consumers
    power_min: f64,
}

impl Node {
    fn get_power(&self) -> f64 {
        self.power
    }

	/// Set `power` to `value`, but limit it to the interval [`power_min`,`power_max`]
    fn set_power(&mut self, value: f64) {
        if value >= self.power_min {
            if value <= self.power_max {
                self.power = value
            } else {
                self.power = self.power_max
            }
        } else {
            self.power = self.power_min
        }
    }
}

fn main() {
    let mut node = Node {
        power: 1_000_000.0,
        power_max: 10_000_000.0,
        power_min: 0.0,
    };
    node.set_power(2_000_000.0);
    println!("The Smart Grid");
    println!("power: {} W", node.get_power());
}
