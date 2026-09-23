struct Vijand {
    gezondheid: i32,
}

impl Vijand {
    fn is_levend(&self) -> bool {
        self.gezondheid > 0
    }
}

fn main() {
    let vijand = Vijand { gezondheid: 1 };
    assert!(vijand.is_levend());
}
