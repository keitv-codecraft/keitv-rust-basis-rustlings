fn zoek_item(gevonden: bool) -> Option<i32> {
    if gevonden { Some(10) } else { None }
}

fn main() {
    assert!(zoek_item(false).is_none());
}
