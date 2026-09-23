struct Kist<T> {
    inhoud: T,
}

fn main() {
    let kist = Kist { inhoud: 100 };
    assert_eq!(kist.inhoud, 100);
}
