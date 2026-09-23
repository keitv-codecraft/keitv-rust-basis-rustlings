fn toon_naam(naam: String) {
    println!("Welkom, {naam}!");
    drop(naam);
}

fn main() {
    let naam = String::from("Arin");
    toon_naam(naam.clone());
    println!("De speler heet {naam}.");
}
