fn toon_tekst(tekst: &str) {
    println!("{tekst}");
}

fn main() {
    let bericht = String::from("De deur gaat open.");
    toon_tekst(&bericht);
}
