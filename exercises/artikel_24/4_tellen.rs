use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert("Arin", 10);
    // TODO: Tel 5 op bij Arin.
    assert_eq!(scores.get("Arin"), Some(&15));
}