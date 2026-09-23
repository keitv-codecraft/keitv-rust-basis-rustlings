#[derive(Debug)]
struct Speler {
    naam: String,
}

fn main() {
    let speler = Speler {
        naam: String::from("Arin"),
    };
    println!("{speler:?}");
    assert_eq!(speler.naam, "Arin");
}
