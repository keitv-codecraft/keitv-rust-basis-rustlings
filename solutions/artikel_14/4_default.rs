#[derive(Default)]
struct Speler {
    gezondheid: i32,
}

fn main() {
    let speler = Speler::default();
    assert_eq!(speler.gezondheid, 0);
}
