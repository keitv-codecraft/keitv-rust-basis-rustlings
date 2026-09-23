fn rang(score: i32) -> &'static str {
    if score >= 500 { "Held" } else { "Beginner" }
}

fn main() {
    assert_eq!(rang(600), "Held");
    assert_eq!(rang(100), "Beginner");
}
