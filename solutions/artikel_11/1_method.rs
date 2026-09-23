struct Speler {
    naam: String,
}

impl Speler {
    fn toon_naam(&self) {
        println!("{}", self.naam);
    }
}

fn main() {
    let speler = Speler {
        naam: String::from("Arin"),
    };
    speler.toon_naam();
}
