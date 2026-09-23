struct Speler {
    naam: String,
    gezondheid: i32,
}

fn main() {
    let speler = Speler {
        naam: String::from("Arin"),
        gezondheid: 100,
    };

    assert_eq!(speler.naam, "Arin");
    assert_eq!(speler.gezondheid, 100);
}
