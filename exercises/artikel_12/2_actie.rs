enum Actie {
    Aanvallen,
    Genezen,
}

fn toon_actie(actie: Actie) {
    // TODO: Gebruik match en print beide mogelijkheden.
    let _ = actie;
}

fn main() {
    toon_actie(Actie::Aanvallen);
}