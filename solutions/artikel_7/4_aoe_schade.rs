fn main() {
    let mut gezondheid = 30;

    for _ in 0..3 {
        gezondheid -= 10;
    }

    assert_eq!(gezondheid, 0);
}
