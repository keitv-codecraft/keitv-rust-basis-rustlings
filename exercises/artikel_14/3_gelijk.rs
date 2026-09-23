#[derive(PartialEq)]
enum Status {
    Menu,
    Spelen,
}

fn main() {
    // TODO: Controleer of deze status Spelen is.
    assert!(Status::Spelen != Status::Menu);
}