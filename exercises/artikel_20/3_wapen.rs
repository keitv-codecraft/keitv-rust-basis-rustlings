fn naam(wapen: Option<&str>) -> String {
    // TODO: Geef de naam of "Geen wapen" terug.
    let _ = wapen;
    String::new()
}

fn main() {
    assert_eq!(naam(None), "Geen wapen");
}