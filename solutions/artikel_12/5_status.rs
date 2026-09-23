#[derive(Clone, Copy)]
enum Status {
    Menu,
    Spelen,
    Pauze,
}

fn naam(status: Status) -> &'static str {
    match status {
        Status::Menu => "Menu",
        Status::Spelen => "Spelen",
        Status::Pauze => "Pauze",
    }
}

fn main() {
    assert_eq!(naam(Status::Menu), "Menu");
    assert_eq!(naam(Status::Spelen), "Spelen");
    assert_eq!(naam(Status::Pauze), "Pauze");
}
