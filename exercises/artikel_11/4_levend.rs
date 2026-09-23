struct Vijand {
    gezondheid: i32,
}

impl Vijand {
    fn is_levend(&self) -> bool {
        // TODO: Geef true terug als gezondheid groter dan 0 is.
        false
    }
}

fn main() {
    let vijand = Vijand { gezondheid: 1 };
    assert!(vijand.is_levend());
}