mod speler {
    pub fn naam() -> &'static str {
        "Arin"
    }
}

fn main() {
    assert_eq!(speler::naam(), "Arin");
}
