mod speler {
    pub fn status() -> &'static str {
        "klaar"
    }
}

fn main() {
    assert_eq!(speler::status(), "klaar");
}
