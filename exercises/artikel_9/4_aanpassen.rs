fn voeg_uitroepteken_toe(bericht: &mut String) {
    // TODO: Voeg een uitroepteken toe.
}

fn main() {
    let mut bericht = String::from("Pas op");
    voeg_uitroepteken_toe(&mut bericht);
    assert_eq!(bericht, "Pas op!");
}