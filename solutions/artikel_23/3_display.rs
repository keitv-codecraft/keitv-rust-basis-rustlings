use std::fmt::Display;

fn toon<T: Display>(waarde: T) {
    println!("{waarde}");
}

fn main() {
    toon(10);
}
