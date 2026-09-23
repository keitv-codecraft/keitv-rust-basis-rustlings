#![allow(clippy::useless_vec)]

fn main() {
    let namen = vec!["Arin", "Borin"];
    let kopie: Vec<String> = namen.iter().map(|naam| String::from(*naam)).collect();
    assert_eq!(kopie, vec!["Arin", "Borin"]);
}
