struct Node {
    power: f64, // current power in watt - Producers have positive power. Consumers have negative power.
    power_max: f64, // maximum power in watt - the peak capacity of producers
}

fn main() {
    let mut node = Node {
        power: 1_000_000.0,
        power_max: 10_000_000.0,
    };
    println!("The Smart Grid");
    println!("power: {} W", node.power);
}
