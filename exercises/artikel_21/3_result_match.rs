fn boodschap(resultaat: Result<i32, String>) -> String {
    // TODO: Geef de waarde of foutmelding terug als tekst.
    let _ = resultaat;
    String::new()
}

fn main() {
    assert_eq!(boodschap(Ok(5)), "Gelukt: 5");
}