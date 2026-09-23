#[allow(clippy::unnecessary_wraps)]
fn wapen() -> Option<String> {
    Some(String::from("Zwaard"))
}

fn main() {
    assert!(wapen().is_some());
}
