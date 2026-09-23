fn eerste<T>(waarden: &[T]) -> Option<&T> {
    waarden.first()
}

fn main() {
    let namen = ["Arin", "Borin"];
    assert_eq!(eerste(&namen), Some(&"Arin"));
}
