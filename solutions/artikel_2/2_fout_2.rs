// Waarom kan Rust `random_range` niet vinden?

// TODO: Probeer het probleem op te lossen.

use rand::random_range;

fn main() {
    let worp = random_range(1..=6);

    println!("Je gooide {worp}!");
}
