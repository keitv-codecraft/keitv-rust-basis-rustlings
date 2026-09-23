#[allow(clippy::vec_init_then_push)]
fn main() {
    let mut items = Vec::new();
    items.push("Zwaard");
    assert_eq!(items.len(), 1);
}
