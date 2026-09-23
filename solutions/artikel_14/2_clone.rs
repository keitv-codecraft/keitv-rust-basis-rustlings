#[derive(Clone)]
struct Wapen {
    naam: String,
}

fn main() {
    let eerste = Wapen {
        naam: String::from("Zwaard"),
    };
    let tweede = eerste.clone();
    assert_eq!(eerste.naam, tweede.naam);
}
