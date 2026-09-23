fn main() {
    let scores = vec![10, 20, 30];
    // TODO: Tel 5 op bij iedere score.
    let hogere: Vec<i32> = scores.iter().map(|score| *score).collect();
    assert_eq!(hogere, vec![15, 25, 35]);
}