#[allow(clippy::unnecessary_wraps)]
fn score() -> Result<i32, String> {
    Ok(100)
}

fn main() {
    assert_eq!(score(), Ok(100));
}
