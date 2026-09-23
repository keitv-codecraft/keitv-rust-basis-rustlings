use std::collections::HashMap;

fn main() {
    let mut inventory = HashMap::new();
    // TODO: Voeg drie drankjes toe.
    assert_eq!(inventory.get("drankje"), Some(&3));
}