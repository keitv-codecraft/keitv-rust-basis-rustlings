fn main() {
    let namen = vec!["Goblin", "Ork", "Draak"];
    // TODO: Controleer of er een Draak is.
    let heeft_draak = namen.iter().any(|naam| *naam == "Draak");
    assert!(heeft_draak);
}