mod speler {
    pub fn status() -> &'static str {
        // TODO: Geef "klaar" terug.
        ""
    }
}

fn main() {
    assert_eq!(speler::status(), "klaar");
}