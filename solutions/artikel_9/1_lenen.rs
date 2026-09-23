fn toon_naam(naam: &String) {
    println!("{naam}");
}

fn main() {
    let naam = String::from("Arin");
    toon_naam(&naam);
}
