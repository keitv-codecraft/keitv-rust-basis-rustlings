// Een magier gooit een vuurbal richting de vijand
// door deze met zijn hand in een rechte lijn weg te slingeren.
// Over de lengte van zijn arm zet de magier kracht om de
// vuurbal te versnellen. Eenmaal losgelaten vervolgt de vuurbal
// zijn weg richting de vijand.
// De vuurbal wordt onderweg niet beinvloed door de zwaartekracht
// en blijft dus zweven, maar hij wordt wel afgeremd door de lucht

// Aan jou nu de taak om te bepalen hoe lang het duurt voordat
// de vuurbal de vijand raakt. Gelukkig staat de vijand stil met
// zijn rug naar de magier. Hij merkt dus niet dat de vuurbal eraan komt.

// De luchtweerstand is een kracht (in Newton) afhankelijk van de
// snelheid (in meters per seconde). Hiervoor kun je de volgende
// functie gebruiken.
fn drag_force(velocity: f64) -> f64 {
    // Op basis van een bolvormig object met straal 10cm
    // wat zich door de lucht beweegt op zeeniveau (1atm)
    0.00904 * velocity * velocity
}

// De magier versnelt de vuurbal door een constante kracht te zetten
// op de vuurbal over een afstand die gelijk is aan zijn armlengte.
// Stel verder dat we weten hoe ver de vijand van de magier staat.
// We hebben nu genoeg informatie om de tijd te bepalen die nodig is
// voordat de vuurbal de vijand bereikt.

// We gaan dit niet analytisch oplossen met natuurkunde, maar met een
// simulatie, net zoals gebeurd in computer games.
fn time_to_hit(
    magician_force: f64, // Newton
    arm_length: f64,     // meter
    fireball_mass: f64,  // kilogram
    enemy_distance: f64, // meter
) -> f64 // seconde
{
    const DELTA_TIME: f64 = 0.0001;
    let mut time = 0.0;
    let mut velocity = 0.0;
    let mut position = 0.0;
    while position < enemy_distance {
        // De magier oefent alleen kracht uit zolang position
        // kleiner is dan arm_length. De luchtweerstand is
        // altijd aanwezig maar is tegengesteld aan de
        // bewegingsrichting.
        // TODO: Bepaal de totale kracht op de vuurbal
        let mut force = ???;

        // Vervolgens bepalen we de versnelling met F = m * a
        let acceleration = force / fireball_mass;

        // Iedere tijdstap zorgt de versnelling voor een verandering in snelheid
        // Bijvoorbeeld, als je versnelt met 10 meter per seconde per seconde
        // dan ga je na 1 seconde 10 meter per seconde sneller. Hoeveel sneller is
        // dit na DELTA_TIME seconde?
        // TODO: bepaal de verandering in snelheid na DELTA_TIME
        velocity += ???;

        // Iedere tijdstap zorgt de snelheid voor een verandering in positie
        // Bijvoorbeeld, als je beweegt met 10 meter per seconde dan beweeg
        // je in 1 seconde 10 meter. Hoeveel meter is dit na DELTA_TIME seconde?
        // TODO: bepaal de verandering in positie na DELTA_TIME
        position += ???;

        // De simulatie is nu 1 tijdstap verder, dat wil zeggen dat de
        // simulatietijd met DELTA_TIME is toegenomen
        time += DELTA_TIME;
    }
    time
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nearby_enemy() {
        let time = time_to_hit(20.0, 1.0, 0.2, 10.0);
        assert!((time - 0.946).abs() < 0.001, "time = {time}");
    }

    #[test]
    fn strong_magician() {
        let time = time_to_hit(30.0, 1.0, 0.2, 20.0);
        assert!((time - 1.894).abs() < 0.001, "time = {time}");
    }

    #[test]
    fn light_fireball() {
        let time = time_to_hit(20.0, 1.0, 0.1, 20.0);
        assert!((time - 2.745).abs() < 0.001, "time = {time}");
    }

    #[test]
    fn short_arms() {
        let time = time_to_hit(20.0, 0.5, 0.2, 20.0);
        assert!((time - 3.264).abs() < 0.001, "time = {time}");
    }
}
