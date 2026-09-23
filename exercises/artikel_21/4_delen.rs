fn deel(getal: i32, deler: i32) -> Result<i32, String> {
    // TODO: Geef een fout als deler nul is.
    let _ = (getal, deler);
    Ok(0)
}

fn main() {
    assert_eq!(deel(10, 2), Ok(5));
    assert!(deel(10, 0).is_err());
}