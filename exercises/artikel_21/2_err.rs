fn controleer(gezondheid: i32) -> Result<(), String> {
    // TODO: Geef Err terug als gezondheid kleiner dan nul is.
    let _ = gezondheid;
    Ok(())
}

fn main() {
    assert!(controleer(-1).is_err());
}