struct Speler {
    naam: String,
}

impl Speler {
    fn bericht(&self) -> String {
        // TODO: Geef een welkomsbericht terug.
        String::new()
    }
}

fn main() {
    let speler = Speler {
        naam: String::from("Arin"),
    };
    assert_eq!(speler.bericht(), "Welkom, Arin!");
}