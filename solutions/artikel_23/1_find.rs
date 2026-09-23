#![allow(clippy::useless_vec)]

fn main() {
    let scores = vec![10, 20, 75, 30];
    let gevonden = scores.iter().find(|score| **score > 50);
    assert_eq!(gevonden, Some(&75));
}
