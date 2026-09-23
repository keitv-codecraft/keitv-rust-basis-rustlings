struct Vijand {
    naam: String,
    gezondheid: i32,
}

fn main() {
    // TODO: Maak een Goblin met 30 gezondheid.
    let vijand = Vijand {
        naam: String::new(),
        gezondheid: 0,
    };
    assert_eq!(vijand.naam, "Goblin");
    assert_eq!(vijand.gezondheid, 30);
}