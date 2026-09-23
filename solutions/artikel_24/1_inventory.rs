use std::collections::HashMap;

fn main() {
    let mut inventory = HashMap::new();
    inventory.insert("drankje", 3);
    assert_eq!(inventory.get("drankje"), Some(&3));
}
