use smartgrid::Node;

fn main() {
    let mut node = Node::new(1_000_000.0, 10_000_000.0, 0.0);
    node.set_power(2_000_000.0);
    println!("The Smart Grid");
    println!("power: {} W", node.get_power());
}
