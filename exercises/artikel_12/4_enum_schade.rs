enum Schade {
    Normaal(i32),
}

fn waarde(schade: Schade) -> i32 {
    // TODO: Haal de hoeveelheid uit de variant.
    let _ = schade;
    0
}

fn main() {
    assert_eq!(waarde(Schade::Normaal(25)), 25);
}