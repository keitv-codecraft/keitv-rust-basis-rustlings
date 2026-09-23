fn eerste_woord(tekst: &str) -> &str {
    tekst.split_whitespace().next().unwrap_or("")
}

fn main() {
    assert_eq!(eerste_woord("De draak"), "De");
}
