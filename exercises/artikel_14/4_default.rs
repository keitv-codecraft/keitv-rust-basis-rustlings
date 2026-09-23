#[derive(Default)]
struct Speler {
    gezondheid: i32,
}

fn main() {
    // TODO: Maak de standaardspeler.
    let speler = Speler { gezondheid: 0 };
    assert_eq!(speler.gezondheid, 0);
}