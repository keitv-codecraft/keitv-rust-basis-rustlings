fn main() {
    // Alle variabelen in Rust hebben een type. Types worden door Rust
    // gecontroleerd om fouten te voorkomen. We komen hier later op terug.
    // Omdat alle expressies ook een type hebben is het meestal mogelijk
    // voor Rust om automatisch af te leiden wat het type van een variabele
    // is. Soms is het toch nodig om zelf aan te geven wat het type is.
    // In zo'n geval zetten we het type achter een dubbele punt : na de
    // variabele naam. Bijvoorbeeld,
    let nice: i32 = 69;

    // Omdat de expressie 69 het type i32 (32-bits signed integer) heeft
    // hoeft je het type hiervoor niet op te schrijven, maar als we 69
    // op willen slaan als een ander type moeten we wel expliciet zijn.
    // Voor simpele gevallen kan dit op twee verschillende manieren:
    let nice2: i64 = 69;
    let nice3 = 69i64;

    // Als je in VsCodium werkt heb je waarschijnlijk de rust-analyzer
    // extensie geinstalleerd. Standaard geeft deze zogenaamde inlay-
    // hints. Dit is de tekst ': i64' die je achter nice3 op regel 16
    // ziet. Het zien van deze type-informatie, zonder deze te hoeven
    // opschrijven verhoogt de leesbaarheid, terwijl de code er niet
    // door vervuild raakt. In de praktijk wordt type-informatie op
    // veel plekken afgeleid. Als je dit overal zou moeten schrijven
    // zou je veel code aan moeten passen om een type te veranderen.
    // Op deze manier blijft het veranderen van types eenvoudig. Later
    // zullen we hier nog meer voorbeelden van zien.

    // Door te dubbelklikken op een inlay-hint wordt deze voor je
    // uitgeschreven in de code zelf. Probeer dit maar eens bij nice3
    // hierboven.

    // TODO: maak hieronder een variabele met een floating-point getal
    //       erin. Standaard wordt dit een f64. Maak hier vervolgens
    //       een f32 van op de twee verschillende manieren die we
    //       hiervoor hebben gezien.
    // let ??? = ???;

    // Laten we nu eens beide typen floats eens printen tot 10 cijfers
    // achter de komma (in Rust schrijven we een punt . volgens de
    // Engelse conventie)
    println!("0.1 als f64: {:.10}", 0.1f64);
    println!("0.1 als f32: {:.10}", 0.1f32);
    // Zie je een verschil?

    // Later komen we hierop terug, maar als je nu al nieuwsgierig bent
    // kun je kijkje nemen op
    // https://nl.wikipedia.org/wiki/Zwevendekommagetal

    // Dit heeft niet zozeer met Rust te maken. Floating point getallen
    // volgen in alle programmeertalen een standaard die bepaalde trade-offs
    // maakt tussen precisie en bereik. Als je een groot getal wilt opslaan
    // dat niet in een integer past, kun je een floating-point getal gebruiken,
    // maar dan verlies je de minst significante informatie.
    // Ook als je een decimaal getal wilt opslaan gebruik je meestal een
    // floating-point getal. Het voordeel van floating-point getallen is dat
    // je er zowel heel grote als heel kleine getallen in kunt opslaan, terwijl
    // je toch de meest significante informatie behoud. Het voorziet daarmee
    // in de directe behoefte die wetenschappers hebben om numerieke data
    // efficient op de comuter op te slaan. Misschien ken je nog wel de
    // wetenschappelijke notatie uit de natuurkunde...
    // https://nl.wikipedia.org/wiki/Wetenschappelijke_notatie
    // Omdat je in games nogal eens 'physics' gebruikt om de wereld te simuleren
    // is het belangrijk dat je weet hoe je floating-point gebruikt.
}

// TODO: verwijder de volgende regel om de exercise op te lossen
static_assertions::const_assert!(false);
