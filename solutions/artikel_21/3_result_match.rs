fn boodschap(resultaat: Result<i32, String>) -> String {
    match resultaat {
        Ok(waarde) => format!("Gelukt: {waarde}"),
        Err(fout) => format!("Fout: {fout}"),
    }
}

fn main() {
    assert_eq!(boodschap(Ok(5)), "Gelukt: 5");
}
