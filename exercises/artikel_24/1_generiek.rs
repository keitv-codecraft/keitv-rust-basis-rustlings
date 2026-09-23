fn eerste<T>(waarden: &[T]) -> Option<&T> {
    // TODO: Geef de eerste waarde terug.
    let _ = waarden;
    None
}

fn main() {
    assert_eq!(eerste(&[10, 20]), Some(&10));
}