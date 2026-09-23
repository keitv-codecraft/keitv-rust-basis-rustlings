trait Levende {
    fn is_levend(&self) -> bool;
}

struct Speler;

impl Levende for Speler {
    fn is_levend(&self) -> bool {
        // TODO: Deze speler leeft altijd.
        false
    }
}

fn main() {
    assert!(Speler.is_levend());
}