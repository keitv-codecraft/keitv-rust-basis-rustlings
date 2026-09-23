use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert("Arin", 100);
    // TODO: Zoek de score op.
    assert_eq!(scores.get("Arin"), Some(&0));
}