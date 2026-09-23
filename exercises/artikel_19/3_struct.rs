mod game {
    pub struct Speler {
        pub gezondheid: i32,
    }
}

fn main() {
    // TODO: Maak een publieke speler met 100 gezondheid.
    let speler = game::Speler { gezondheid: 0 };
    assert_eq!(speler.gezondheid, 100);
}