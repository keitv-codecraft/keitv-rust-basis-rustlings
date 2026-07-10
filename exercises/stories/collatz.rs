// Er is een interessant onopgelost probleem in de wiskunde dat bijzonder eenvoudig
// uitgelegd kan worden. We beginnen door een willekeurig positief geheel getal te kiezen.
// Als dit getal even (deelbaar door 2) is dan delen we het door 2, maar als het getal
// oneven (niet deelbaar door 2) is dat vermenigvuldigen we het met 3 en tellen er 1 bij op.
// Met dit nieuwe getal doen we weer hetzelfde: even -> deel door 3, oneven -> keer 3 plus 1.
// Dit herhalen we net zo lang totdat we bij het getal 1 uitkomen.

// Laten we naar enkele voorbeelden kijken:
// 3 (oneven, dus keer 3 plus 1) -> 10 (even, delen door 2) -> 5 -> 16 -> 8 -> 4 -> 2 -> 1
// 7 -> 22 -> 11 -> 34 -> 17 -> 52 -> 26 -> 13 -> 40 -> 20 -> 10 (al gezien) -> ... -> 1
// 9 -> 28 -> 14 -> 7 (al gezien) -> ... -> 1
// Wat gebeurd er als we met 1 beginnen? 1 -> 4 -> 2 -> 1 (We komen in een lus terecht)

// We zien in deze voorbeelden dat we steeds weer uiteindelijk bij 1 terecht komen.
// Het vermoeden dat dit voor alle positieve gehele getallen geldt heet het vermoeden van Collatz,
// omdat de wiskundige Lothar Collatz de eerste was die het beschreef.
// Het is tot op de dag van vandaag onbekend of het vermoeden waar is.
// Velen hebben geprobeerd om het te bewijzen, maar het is nog niemand gelukt.
// Misschien is het vermoeden helemaal niet waar! Als we een getal kunnen vinden dat niet
// uiteindelijk naar 1 gaat dan hebben we een tegenvoorbeeld te pakken.

// Er staat inmiddels een geldbedrag van meer dan een half miljoen euro tegenover als je het vermoeden
// kunt bewijzen of een tegenvoorbeeld kunt vinden. https://mathprize.net/posts/collatz-conjecture/
// We zijn dan wel geen wiskundigen, maar wel programmeurs, dus wie weet kunnen we een
// tegenvoorbeeld vinden.

// Laten we beginnen met het bepalen van volgende getal in de reeks door een functie te schrijven
fn next_collatz(n: u128) -> u128 {
    // TODO: als n even is geven we de helft terug en als n oneven is geven 3*n+1 terug
    if n.is_multiple_of(2) {
        n / 2
    } else {
        3 * n + 1
    }
}

// Vervolgens gaan we bepalen of een getal uiteindelijk naar 1 gaat en zo ja, in hoeveel stappen
fn step_count(n: u128) -> Option<usize> {
    if n == 1 {
        Some(0)
    } else if let Some(steps) = step_count(next_collatz(n)) {
        Some(1 + steps)
    } else {
        None
    }
}

// Onze step_count functie gebruikt recursie: de functie roept zichzelf aan.
// Iedere keer dat de functie zichzelf aanroept moet de vorige toestand van
// de variabelen worden opgeslagen. Als een functie zichzelf heel vaak aanroept
// hebben we hiervoor niet voldoende geheugen. TODO: Probeer eens de
// big_step_count test hieronder te activeren en kijk wat er gebeurt.

// Laten we proberen om recursie te vermijden door een rechtstreekse implementatie
// te schrijven met een loop.
fn step_count_nonrecursive(mut n: u128) -> Option<usize> {
    let mut step_count = 0;
    while n != 1 {
        n = next_collatz(n);
        step_count += 1;
    }
    Some(step_count)
}

// Valt je iets op aan deze implementatie?
// We geven altijd de Some variant terug, maar we zijn juist op zoek naar een
// tegenvoorbeeld, dus zou het logisch zijn als we detecteren wanneer we
// niet op 1 uitkomen.
// Hiervoor bestaan twee mogelijkheden:
// 1. We komen in een andere lus dan 1 -> 4 -> 2 -> 1 terecht
// 2. De getallen worden zo groot dat ze niet meer in een u128 passen
// In beide gevallen zal onze while loop eindeloos blijven doorgaan.
// Terug naar de tekentafel dus. Aan jou de taak om een veilige versie
// van deze functie te schrijven die gegarandeerd stopt.
fn step_count_safe(mut n: u128) -> Option<usize> {
    // TODO: implementeer een functie die het aantal stappen telt, maar die
    // stopt wanneer we in een lus terechtkomen of wanneer het resultaat
    // van next_collatz niet meer in een u128 past.
    // Merk op dat de huidige implementatie van next_collatz niet veilig is.
    // Immers 3*n + 1 kan groter worden dat u128::MAX. Maak dus ook een
    // nieuwe next_collatz_safe functie die een Option<u128> teruggeeft en
    // gebruik die functie voor deze implementatie.
    todo!()
}

fn main() {
    // TODO: Gebruik je nieuwe step_count_safe functie om op zoek te gaan naar
    // een tegenvoorbeeld voor het vermoeden van Collatz
    // Is je implementatie snel genoeg om veel getallen te controleren?
    // Zo niet, kun je het sneller maken?
}

#[cfg(test)]
mod tests {
    use core::assert_eq;

    use super::*;

    #[test]
    fn simple_steps() {
        assert_eq!(next_collatz(0), 0);
        assert_eq!(next_collatz(1), 4);
        assert_eq!(next_collatz(2), 1);
    }

    #[test]
    fn small_step_count() {
        assert_eq!(step_count(1), Some(0));
        assert_eq!(step_count(2), Some(1));
        assert_eq!(step_count(3), Some(7));
        assert_eq!(step_count(4), Some(2));
    }

    // TODO: activeer deze test tijdelijk om te zien wat er gebeurt
    // #[test]
    // fn big_step_count() {
    //     assert_eq!(step_count_safe(837799), Some(524));
    // }

    #[test]
    fn bigger_step_count() {
        assert_eq!(step_count_nonrecursive(1), Some(0));
        assert_eq!(step_count_nonrecursive(2), Some(1));
        assert_eq!(step_count_nonrecursive(3), Some(7));
        assert_eq!(step_count_nonrecursive(4), Some(2));
        assert_eq!(step_count_nonrecursive(837_799), Some(524));
    }

    #[test]
    fn safe_step_count() {
        assert_eq!(step_count_safe(1), Some(0));
        assert_eq!(step_count_safe(2), Some(1));
        assert_eq!(step_count_safe(3), Some(7));
        assert_eq!(step_count_safe(4), Some(2));
        assert_eq!(step_count_safe(837_799), Some(524));
        assert_eq!(step_count_safe(0), None);
    }
}
