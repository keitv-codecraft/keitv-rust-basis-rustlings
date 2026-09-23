struct Speler {
    naam: String,
}

impl Speler {
    fn bericht(&self) -> String {
        format!("Welkom, {}!", self.naam)
    }
}

fn main() {
    let speler = Speler {
        naam: String::from("Arin"),
    };
    assert_eq!(speler.bericht(), "Welkom, Arin!");
}
