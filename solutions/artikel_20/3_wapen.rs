fn naam(wapen: Option<&str>) -> String {
    match wapen {
        Some(naam) => String::from(naam),
        None => String::from("Geen wapen"),
    }
}

fn main() {
    assert_eq!(naam(None), "Geen wapen");
}
