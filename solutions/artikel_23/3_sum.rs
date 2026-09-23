#![allow(clippy::useless_vec)]

fn main() {
    let schade = vec![10, 20, 5];
    let totaal: i32 = schade.iter().sum();
    assert_eq!(totaal, 35);
}
