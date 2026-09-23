fn eerste<T>(waarden: &[T]) -> Option<&T> {
    waarden.first()
}

fn main() {
    assert_eq!(eerste(&[10, 20]), Some(&10));
}
