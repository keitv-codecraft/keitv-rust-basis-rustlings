mod game {
    pub struct Speler {
        pub gezondheid: i32,
    }
}

fn main() {
    let speler = game::Speler { gezondheid: 100 };
    assert_eq!(speler.gezondheid, 100);
}
