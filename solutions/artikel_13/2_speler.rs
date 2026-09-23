trait Levende {
    fn is_levend(&self) -> bool;
}

struct Speler {
    gezondheid: i32,
}

impl Levende for Speler {
    fn is_levend(&self) -> bool {
        self.gezondheid > 0
    }
}

fn main() {
    assert!(Speler { gezondheid: 10 }.is_levend());
}
