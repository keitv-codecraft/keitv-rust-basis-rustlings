#[derive(Clone, Copy)]
enum Actie {
    Aanvallen,
    Genezen,
}

fn toon_actie(actie: Actie) {
    match actie {
        Actie::Aanvallen => println!("Aanvallen"),
        Actie::Genezen => println!("Genezen"),
    }
}

fn main() {
    toon_actie(Actie::Aanvallen);
    toon_actie(Actie::Genezen);
}
