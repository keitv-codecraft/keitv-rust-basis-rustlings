trait Beschrijfbaar {
    fn beschrijving(&self) -> &'static str;
}

struct Speler;

impl Beschrijfbaar for Speler {
    fn beschrijving(&self) -> &'static str {
        "Speler"
    }
}

fn main() {
    assert_eq!(Speler.beschrijving(), "Speler");
}
