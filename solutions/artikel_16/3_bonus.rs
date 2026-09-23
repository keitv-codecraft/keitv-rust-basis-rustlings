fn main() {
    let bonus = 10;
    let geef_bonus = |score: i32| score + bonus;
    assert_eq!(geef_bonus(100), 110);
}
