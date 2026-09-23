#[derive(Debug, PartialEq)]
enum Status {
    Menu,
    Spelen,
}

fn main() {
    let spelen = Status::Spelen;
    let menu = Status::Menu;
    assert_ne!(spelen, menu);
}
