fn main() {
    let scores = vec![10, 60, 30, 90];
    // TODO: Houd scores van 50 of hoger over.
    let hoge: Vec<&i32> = scores.iter().filter(|score| **score < 0).collect();
    assert_eq!(hoge, vec![&60, &90]);
}