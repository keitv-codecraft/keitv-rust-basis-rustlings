struct Vijand {
    naam: String,
    gezondheid: i32,
}

fn main() {
    let vijand = Vijand {
        naam: String::from("Goblin"),
        gezondheid: 30,
    };
    assert_eq!(vijand.naam, "Goblin");
    assert_eq!(vijand.gezondheid, 30);
}
