mod speler {
    // TODO: Maak deze functie publiek.
    fn naam() -> &'static str {
        "Arin"
    }
}

fn main() {
    assert_eq!(speler::naam(), "Arin");
}