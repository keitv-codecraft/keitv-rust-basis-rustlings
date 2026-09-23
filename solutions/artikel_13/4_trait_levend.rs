trait Levende {
    fn is_levend(&self) -> bool;
}

struct Speler;

impl Levende for Speler {
    fn is_levend(&self) -> bool {
        true
    }
}

fn main() {
    assert!(Speler.is_levend());
}
