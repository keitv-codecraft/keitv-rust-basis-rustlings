fn main() {
    let namen = vec!["Arin", "Borin"];
    // TODO: Verzamel de namen in een nieuwe Vec<String>.
    let kopie: Vec<String> = namen.iter().map(|naam| String::new()).collect();
    assert_eq!(kopie, vec!["Arin", "Borin"]);
}