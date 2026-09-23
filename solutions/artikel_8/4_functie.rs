fn toon_wapen(wapen: String) {
    println!("Wapen: {wapen}");
    drop(wapen);
}

fn main() {
    let wapen = String::from("Boog");
    toon_wapen(wapen);
}
