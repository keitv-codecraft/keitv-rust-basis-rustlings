fn main() {
    let scores = vec![10, 20, 30];
    let mut totaal = 0;

    for score in &scores {
        totaal += score;
    }

    assert_eq!(totaal, 60);
}
