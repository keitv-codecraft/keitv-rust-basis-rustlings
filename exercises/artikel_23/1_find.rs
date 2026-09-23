fn main() {
    let scores = vec![10, 20, 75, 30];
    // TODO: Vind de eerste score boven 50.
    let gevonden = scores.iter().find(|score| **score > 50);
    assert_eq!(gevonden, Some(&75));
}