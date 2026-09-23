use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert("Arin", 10);
    if let Some(score) = scores.get_mut("Arin") {
        *score += 5;
    }
    assert_eq!(scores.get("Arin"), Some(&15));
}
