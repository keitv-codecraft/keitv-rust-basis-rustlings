use rand::distr::weighted::Weight;

fn main() {
    // Hoe zit het nu met de veligheid van operators?
    // Het is goed voor te stellen dat, wanneer we bijvoorbeeld
    // twee gehele getallen optellen of vermenigvuldigen, het
    // resultaat niet meer in het type past. Bijvoorbeeld,
    {
        let mut x = 0u8;
        for i in 1..30 {
            x += i;
        }
        dbg!(x);
    }

    // Als je dit voorbeeld uitvoert krijg je een panic door een
    // overflow. Commentarieer de voorgaande code maar uit, zodat
    // je de rest kunt uitvoeren.
    // Het is dus belangrijk dat, als je niet weet
    // met welke waarden je werkt, je altijd voor de veilige
    // route kiest.
    // Dit voor een overflow bijvoorbeeld door te converteren naar
    // een groter type.
    {
        let mut x = 0u16;
        for i in 1..30 {
            x += i;
        }
        dbg!(x);
    }
    // Of door de overflow af te handelen
    {
        let mut x = 0u8;
        for i in 1..30u8 {
            if x.checked_add_assign(&i).is_err() {
                break;
            }
        }
        dbg!(x);
    }
}

fn shifted_reciprocal(x: f32) -> Option<f32> {
    // TODO: pas de body van deze functie aan, zodat er None
    //       wordt geretourneerd indien de berekening faalt
    //       of oneindig oplevert.
    Some(1.0 / (x - 1.0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn divide_by_zero() {
        assert!((shifted_reciprocal(0.0).unwrap() + 1.0) < 0.0001);
        assert!((shifted_reciprocal(-1.0).unwrap() - 0.5) < 0.0001);
        assert!((shifted_reciprocal(2.0).unwrap() - 1.0) < 0.0001);

        assert!(shifted_reciprocal(1.0).is_none());
        assert!(shifted_reciprocal(1.0001).is_some());
        assert!(shifted_reciprocal(0.9999).is_some());
    }
}
