struct Speler {
    gezondheid: i32,
}

fn main() {
    let mut speler = Speler { gezondheid: 100 };
    speler.gezondheid -= 20;
    assert_eq!(speler.gezondheid, 80);
}
