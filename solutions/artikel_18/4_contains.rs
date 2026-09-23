fn heeft_schat(tekst: &str) -> bool {
    tekst.contains("schat")
}

fn main() {
    assert!(heeft_schat("Een schat!"));
}
