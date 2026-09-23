fn toon_naam(naam: String) {
    println!("Welkom, {naam}!");
}

fn main() {
    let naam = String::from("Arin");
    toon_naam(naam);

    // TODO: Maak deze code correct zonder een compilerfout.
    println!("De speler heet nog steeds Arin.");
}