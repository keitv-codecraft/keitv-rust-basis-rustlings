struct Speler {
    gezondheid: i32,
}

impl Speler {
    fn genees(&mut self, hoeveelheid: i32) {
        // TODO: Tel de hoeveelheid bij gezondheid op.
    }
}

fn main() {
    let mut speler = Speler { gezondheid: 40 };
    speler.genees(20);
    assert_eq!(speler.gezondheid, 60);
}