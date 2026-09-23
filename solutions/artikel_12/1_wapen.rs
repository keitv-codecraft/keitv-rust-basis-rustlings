enum Wapen {
    Zwaard,
    Boog,
}

fn main() {
    let wapen = Wapen::Zwaard;
    let boog = Wapen::Boog;

    match (wapen, boog) {
        (Wapen::Zwaard, Wapen::Boog) => println!("Twee wapens"),
        _ => println!("Andere wapens"),
    }
}
