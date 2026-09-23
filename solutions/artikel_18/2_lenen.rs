fn lengte(tekst: &str) -> usize {
    tekst.len()
}

fn main() {
    assert_eq!(lengte("Hallo"), 5);
}
