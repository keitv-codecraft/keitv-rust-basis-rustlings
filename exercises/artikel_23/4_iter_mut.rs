fn main() {
    let mut gezondheid = vec![30, 50];
    // TODO: Genees iedere vijand met 5 gezondheid.
    for waarde in &mut gezondheid {
        *waarde += 5;
    }
    assert_eq!(gezondheid, vec![35, 55]);
}