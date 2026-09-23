fn main() {
    let bonus = 10;
    // TODO: Maak een closure die bonus toevoegt.
    let geef_bonus = |score: i32| score;
    assert_eq!(geef_bonus(100), 110);
}