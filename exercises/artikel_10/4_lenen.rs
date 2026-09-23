struct Speler {
    naam: String,
}

fn toon_naam(speler: &Speler) {
    // TODO: Print de naam.
}

fn main() {
    let speler = Speler {
        naam: String::from("Arin"),
    };
    toon_naam(&speler);
}