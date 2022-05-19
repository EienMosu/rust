const STARTING_MISSILES: i32 = 8;
const READY_AMOUTH: i32 = 2;

fn main() {
    let (mut missiles, mut ready) = (8, 2);
    println!("Firing {} of my {} missiles...", ready, missiles);

    missiles = missiles - ready;
    println!("{} missiles left.", missiles);

    missiles = STARTING_MISSILES;
    ready = READY_AMOUTH;
    println!("{} missiles and {} ready", missiles, ready);
}
