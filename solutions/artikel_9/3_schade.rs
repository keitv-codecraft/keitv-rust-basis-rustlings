fn neem_schade(gezondheid: &mut i32, schade: i32) {
    *gezondheid -= schade;
}

fn main() {
    let mut gezondheid = 100;
    neem_schade(&mut gezondheid, 25);
    assert_eq!(gezondheid, 75);
}
