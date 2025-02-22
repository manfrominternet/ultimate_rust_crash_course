const STARTING_MISSILES: i32 = 8;

fn main() {
    let mut missiles = 8;
    let ready = 2;
    println!("Firing {} of my {} missiles...", ready, missiles);
    missiles = missiles - ready;
    println!("{}", STARTING_MISSILES);
}