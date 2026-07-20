fn main() {
    // Soms willen we de waarden van variabelen kunnen veranderen.
    // Hiervoor gebruiken we het `mut` keyword. Bijvoorbeeld,
    let mut counter = 0;
    counter += 1;
    dbg!(counter);
    // Je kunt het dbg! macro gebruiken om makkelijk een variabele en
    // diens naam te printen. dbg is een afkorting voor debug:
    // het ontdoen van bugs. Als er een probleem met je code is wil
    // je vaak weten wat de waarden van bepaalde veriabeles is op
    // verschillende plekken in je code. dbg! helpt je dit snel te doen.

    // Ook wanneer je een variabele wilt doorgeven aan een functie,
    // zodat die functie de waarde kan veranderen, gebruiken we `mut`.

    // Een variabele heeft altijd precies 1 eigenaar (owner).
    // Als de variabele muteerbaar (mut) is dan is er altijd hoogstens
    // 1 bewerker (mutator).
    // Alleen wanneer er geen bewerkers zijn kunnen er lezers van
    // de variabele bestaan.

    // TODO: los het probleem in deze scope (code tussen {}) op.
    {
        let mut owner = 1;
        let mutator = &mut owner;
        *mutator += 2;
        let reader = &owner;
        dbg!(reader);
    }

    // TODO: los het probleem in deze scope op.
    {
        let mut owner = 1;
        let mutator = &mut owner;
        *mutator += 3;
        dbg!(owner);
    }

    // TODO: los het probleem in deze scope op.
    {
        let owner = Box::new(1);
        let reader = &owner;
        let new_owner = owner.clone();
        dbg!(new_owner);
        dbg!(reader);
    }

    // Box is een type dat wijst naar een willekeurig ander type T
    // Het verschil met een gewone instantie van T is dat deze op
    // de heap in plaats van de stack wordt gemaakt.
    // We komen hier later nog op terug, maar ik gebruik het in het
    // vorige voorbeeld om duidelijk te maken dat een variabele
    // niet meerdere owners kan hebben. Voor de meeste types die je
    // gezien hebt geldt echter dat op regel 43 een kopie zou worden
    // gemaakt, zodat dit probleem zich niet manifesteert.
}
