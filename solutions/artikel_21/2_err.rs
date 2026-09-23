fn controleer(gezondheid: i32) -> Result<(), String> {
    if gezondheid < 0 {
        Err(String::from("Ongeldige gezondheid"))
    } else {
        Ok(())
    }
}

fn main() {
    assert!(controleer(-1).is_err());
}
