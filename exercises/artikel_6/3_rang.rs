fn rang(score: i32) -> &'static str {
    // TODO: Geef "Held" terug bij 500 of meer, anders "Beginner".
    ""
}

fn main() {
    assert_eq!(rang(600), "Held");
    assert_eq!(rang(100), "Beginner");
}