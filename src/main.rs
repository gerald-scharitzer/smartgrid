use smartgrid::Node;

fn main() {
    let mut source = Node::new(1_000_000.0, 10_000_000.0, 0.0);
    source.set_power(2_000_000.0);
    let mut sink = Node::new(-1_000_000.0, 0.0, -5_000_000.0);
    sink.set_power(-2_000_000.0);
    println!("The Smart Grid");
    println!("source: {} W", source.get_power());
    println!("sink: {} W", sink.get_power());
}
