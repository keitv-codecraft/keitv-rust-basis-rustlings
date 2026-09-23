fn eerste<T>(waarden: &[T]) -> Option<&T> {
    waarden.first()
}

fn main() {
    let namen = ["Arin", "Borin"];
    // TODO: Controleer de eerste naam.
    assert_eq!(eerste(&namen), Some(&""));
}