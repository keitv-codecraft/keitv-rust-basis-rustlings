fn score() -> Result<i32, String> {
    // TODO: Geef Ok(100) terug.
    Err(String::from("nog niet"))
}

fn main() {
    assert_eq!(score(), Ok(100));
}