#![allow(clippy::useless_vec)]

fn main() {
    let scores = vec![10, 60, 30, 90];
    let hoge: Vec<&i32> = scores.iter().filter(|score| **score >= 50).collect();
    assert_eq!(hoge, vec![&60, &90]);
}
