trait Levende {
    fn is_levend(&self) -> bool;
}

struct Vijand {
    gezondheid: i32,
}

impl Levende for Vijand {
    fn is_levend(&self) -> bool {
        self.gezondheid > 0
    }
}

fn main() {
    assert!(!Vijand { gezondheid: 0 }.is_levend());
}
