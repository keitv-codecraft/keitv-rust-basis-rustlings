enum Status {
    Menu,
    Spelen,
    Pauze,
}

fn naam(status: Status) -> &'static str {
    // TODO: Geef voor iedere status een tekst terug.
    let _ = status;
    ""
}

fn main() {
    assert_eq!(naam(Status::Pauze), "Pauze");
}