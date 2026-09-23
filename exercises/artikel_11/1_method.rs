struct Speler {
    naam: String,
}

impl Speler {
    // TODO: Maak een method `toon_naam` met `&self`.
}

fn main() {
    let speler = Speler {
        naam: String::from("Arin"),
    };
    speler.toon_naam();
}