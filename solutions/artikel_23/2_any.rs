#![allow(clippy::useless_vec, clippy::manual_contains)]

fn main() {
    let namen = vec!["Goblin", "Ork", "Draak"];
    let heeft_draak = namen.iter().any(|naam| *naam == "Draak");
    assert!(heeft_draak);
}
