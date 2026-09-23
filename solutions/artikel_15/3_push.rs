#[allow(clippy::vec_init_then_push)]
fn main() {
    let mut scores = Vec::new();
    scores.push(10);
    scores.push(20);
    scores.push(30);
    assert_eq!(scores, vec![10, 20, 30]);
}
