const STARTING_MISSILES: i32 = 8;
const READY_AMOUTH: i32 = 2;

fn main() {
    let _ozkan = 6; //if you wanna write unused variables put "_" to begining!
    let (mut missiles, mut ready) = (8, 2);
    println!("Firing {} of my {} missiles...", ready, missiles);
    println!("{} missiles left.", missiles - ready);

    missiles = STARTING_MISSILES;
    ready = READY_AMOUTH;
    println!("{} missiles and {} ready", missiles, ready);
}
