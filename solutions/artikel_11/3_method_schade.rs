struct Speler {
    gezondheid: i32,
}

impl Speler {
    fn neem_schade(&mut self, schade: i32) {
        self.gezondheid -= schade;
    }
}

fn main() {
    let mut speler = Speler { gezondheid: 100 };
    speler.neem_schade(30);
    assert_eq!(speler.gezondheid, 70);
}
