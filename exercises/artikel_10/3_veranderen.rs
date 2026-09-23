struct Speler {
    gezondheid: i32,
}

fn main() {
    let mut speler = Speler { gezondheid: 100 };
    // TODO: Trek 20 gezondheid af.
    assert_eq!(speler.gezondheid, 80);
}