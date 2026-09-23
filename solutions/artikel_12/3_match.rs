#[derive(Clone, Copy)]
enum Status {
    Spelen,
    GameOver,
}

fn is_actief(status: Status) -> bool {
    match status {
        Status::Spelen => true,
        Status::GameOver => false,
    }
}

fn main() {
    assert!(is_actief(Status::Spelen));
    assert!(!is_actief(Status::GameOver));
}
