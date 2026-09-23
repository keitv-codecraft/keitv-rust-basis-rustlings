fn eerste_positief(getallen: &[i32]) -> Option<i32> {
    // TODO: Geef het eerste positieve getal terug.
    let _ = getallen;
    None
}

fn main() {
    assert_eq!(eerste_positief(&[-2, 0, 5]), Some(5));
}