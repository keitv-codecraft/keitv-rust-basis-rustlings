struct Kist<T> {
    inhoud: T,
}

fn main() {
    // TODO: Maak een Kist met het getal 100.
    let kist = Kist { inhoud: 0 };
    assert_eq!(kist.inhoud, 100);
}