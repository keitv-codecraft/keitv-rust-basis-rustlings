fn toon_twee_keer(tekst: &str) {
    println!("{tekst}");
    println!("{tekst}");
}

fn main() {
    let bericht = String::from("Hallo!");
    toon_twee_keer(&bericht);
    toon_twee_keer(&bericht);
}
