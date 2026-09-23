enum Status {
    Spelen,
    GameOver,
}

fn is_actief(status: Status) -> bool {
    // TODO: Geef false terug voor GameOver.
    let _ = status;
    false
}

fn main() {
    assert!(is_actief(Status::Spelen));
    assert!(!is_actief(Status::GameOver));
}