use std::collections::HashMap;

fn main() {
    let mut verslagen = HashMap::new();
    // TODO: Tel twee Goblins met entry.
    assert_eq!(verslagen.get("Goblin"), Some(&2));
}