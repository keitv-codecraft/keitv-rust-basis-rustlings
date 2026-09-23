#![allow(clippy::useless_vec)]

fn main() {
    let mut gezondheid = vec![30, 50];
    for waarde in &mut gezondheid {
        *waarde += 5;
    }
    assert_eq!(gezondheid, vec![35, 55]);
}
