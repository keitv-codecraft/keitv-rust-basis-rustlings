fn main() {
    let schade = vec![10, 20, 5];
    // TODO: Tel alle schade op.
    let totaal: i32 = schade.iter().sum();
    assert_eq!(totaal, 35);
}