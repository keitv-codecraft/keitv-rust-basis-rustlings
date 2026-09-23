#[derive(Clone, Copy)]
enum Schade {
    Normaal(i32),
}

fn waarde(schade: Schade) -> i32 {
    match schade {
        Schade::Normaal(hoeveelheid) => hoeveelheid,
    }
}

fn main() {
    assert_eq!(waarde(Schade::Normaal(25)), 25);
}
