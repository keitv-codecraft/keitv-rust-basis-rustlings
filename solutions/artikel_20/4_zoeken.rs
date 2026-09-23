fn eerste_positief(getallen: &[i32]) -> Option<i32> {
    for getal in getallen {
        if *getal > 0 {
            return Some(*getal);
        }
    }
    None
}

fn main() {
    assert_eq!(eerste_positief(&[-2, 0, 5]), Some(5));
}
