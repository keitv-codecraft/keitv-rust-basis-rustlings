use std::collections::HashMap;

fn main() {
    let mut verslagen = HashMap::new();
    *verslagen.entry("Goblin").or_insert(0) += 1;
    *verslagen.entry("Goblin").or_insert(0) += 1;
    assert_eq!(verslagen.get("Goblin"), Some(&2));
}
