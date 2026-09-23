fn deel(getal: i32, deler: i32) -> Result<i32, String> {
    if deler == 0 {
        Err(String::from("Delen door nul kan niet"))
    } else {
        Ok(getal / deler)
    }
}

fn main() {
    assert_eq!(deel(10, 2), Ok(5));
    assert!(deel(10, 0).is_err());
}
