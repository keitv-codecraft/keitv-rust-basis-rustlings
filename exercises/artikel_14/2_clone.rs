#[derive(Clone)]
struct Wapen {
    naam: String,
}

fn main() {
    let eerste = Wapen {
        naam: String::from("Zwaard"),
    };
    // TODO: Maak tweede met clone.
    let tweede = Wapen {
        naam: String::new(),
    };
    assert_eq!(eerste.naam, tweede.naam);
}