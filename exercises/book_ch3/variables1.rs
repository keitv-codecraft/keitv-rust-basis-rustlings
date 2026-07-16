fn main() {
    // Gebruik het `let` keyword om een nieuwe variable te maken. Bijvoorbeeld,
    let variable_name = "variable_value";

    // We kunnen de waarde van deze variabele zichtbaar maken in de terminal
    // door de println! macro te gebruiken. Bijvoorbeeld,
    println!("variable_name heeft te waarde {variable_name}");
    // Voor de liefhebber: een macro is geen functie, maar eigenlijk een code
    //   generator, zodat je minder hoeft te schrijven.
    //   Ga met de tekstcursor maar eens op het woordje println hierboven staan
    //   en druk vervolgens op CTRL+. Er verschijnt nu een contextmenu.
    //   Kies hier voor de optie "Inline macro" en kijk maar eens wat er gebeurd.
    //   Dit kun je doen voor iedere macro. Macro's herken je aan het uitroepteken
    //   aan het einde van de naam.

    // Het schrijven van een variabele naam tussen accolades {} binnen een string ""
    // heet string interpolatie en is alleen mogelijk bij macro's die dit
    // ondersteunen. Het volgende werkt bijvoorbeeld niet.
    let _ = "variable_name heeft te waarde {variable_name}";

    // Als je goed oplet zie je dat ik de variabele naam hiervoor een underscore _
    // laat zijn. Als je dat doet geef je in Rust aan dat je die variabele
    // expres niet gebruikt. Dit kun je doen om een waarschuwing op te lossen.
    // TODO: Maak hier tijdelijk een nieuwe variabele aan die je niet gebruikt.
    //       Sla vervolgens dit bestand op kijk wat er gebeurd.
    // let ??? = ???;

    // Als het goed is zie je een geel golvend lijntje onder de naam van de variabele.
    // Dit betekent dat er een waarschuwing wordt gegeven door Rust. In dit geval
    // omdat je de variabele niet gebruikt. Dit kun je zien door te lezen wat er in
    // het popup menu verschijnt als je met de muis de variabele naam aanwijst.
    // Dit werkt ook voor rode golvende lijntjes. Deze geven een fout in je code aan.
    // TODO: Gebruik de variabele die je hiervoor hebt gemaakt om de waarschuwing
    //       op te lossen. Merk op dat het gele lijntje pas verdwijnt als je opslaat.
    // println!("Mijn variabele is {???}");

    // String interpolatie werkt alleen met variabelen, en dus niet met bijvoorbeeld
    // expressies. Een expressie is een stukje code dat een waarde oplevert.
    // Expressies vind je vaak aan de rechterkant van een is-gelijk teken in Rust.
    // Bijvoorbeeld, `2+3` is een expressie die je kunt gebruiken om een variabele
    // te maken. Bijvoorbeeld,
    let sum = 2 + 3;

    // TODO: uncomment tijdelijk de volgende regel om te zien dat je niet mag
    //       string interpoleren met een expressie
    // println!("De som van 2 en 3 is {2 + 3}");

    // Oplossing hiervoor is simpel: geef de expressie mee als extra argument van
    // de macro en gebruik lege accolades {} om aan te geven waar de waarde van
    // de expressie ingevuld moet worden. Bijvoorbeeld,
    println!("De som van 2 en 3 is {}", 2 + 3);

    // Je kunt natuurlijk ook gewoon het resultaat van de expressie eerst opslaan
    // in een variabele en deze vervolgens gebruiken in de interpolatie.
    // TODO: Probeer dat hieronder zelf te doen, zonder terug te kijken 😉
    // let ???
    // println! ???
}

// TODO: verwijder de volgende regel om de exercise op te lossen
static_assertions::const_assert!(false);
