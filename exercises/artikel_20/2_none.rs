fn zoek_item(gevonden: bool) -> Option<i32> {
    // TODO: Geef Some(10) bij true en None bij false.
    let _ = gevonden;
    None
}

fn main() {
    assert!(zoek_item(false).is_none());
}