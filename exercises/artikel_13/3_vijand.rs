trait Levende {
    fn is_levend(&self) -> bool;
}

struct Vijand {
    gezondheid: i32,
}

impl Levende for Vijand {
    fn is_levend(&self) -> bool {
        // TODO: Geef aan of de vijand nog leeft.
        false
    }
}

fn main() {
    assert!(!Vijand { gezondheid: 0 }.is_levend());
}