fn aanval(gezondheid: &mut i32, schade: i32) {
    *gezondheid = (*gezondheid - schade).max(0);
}

fn main() {
    let mut gezondheid = 20;
    aanval(&mut gezondheid, 30);
    assert_eq!(gezondheid, 0);
}
