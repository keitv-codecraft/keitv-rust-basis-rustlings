fn bereken_schade(kracht: i32, wapenschade: i32) -> i32 {
    kracht + wapenschade
}

fn main() {
    let schade = bereken_schade(10, 7);
    assert_eq!(schade, 17);
}
