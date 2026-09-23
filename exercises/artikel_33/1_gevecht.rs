fn aanval(gezondheid: &mut i32, schade: i32) {
    // TODO: Trek schade af, maar ga niet onder nul.
}

fn main() {
    let mut gezondheid = 20;
    aanval(&mut gezondheid, 30);
    assert_eq!(gezondheid, 0);
}