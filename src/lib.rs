pub struct Node {
	/// current power in watt - Producers have positive power. Consumers have negative power.
    power: f64,
	/// maximum power in watt - the peak capacity of producers
    power_max: f64,
	/// minimum power in watt - the peak capacity of consumers
    power_min: f64,
}

impl Node {
    pub fn new(power: f64, power_max: f64, power_min: f64) -> Node {
        Node {
            power,
            power_max,
            power_min,
        }
    }

    pub fn get_power(&self) -> f64 {
        self.power
    }

	/// Set `power` to `value`, but limit it to the interval [`power_min`,`power_max`]
    pub fn set_power(&mut self, value: f64) {
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
