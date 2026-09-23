#![allow(clippy::useless_vec)]

fn main() {
    let scores = vec![10, 20, 30];
    let hogere: Vec<i32> = scores.iter().map(|score| *score + 5).collect();
    assert_eq!(hogere, vec![15, 25, 35]);
}
