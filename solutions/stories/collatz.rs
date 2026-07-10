use std::collections::{HashMap, HashSet};

fn step_count_safe(mut n: u128) -> Option<usize> {
    let mut step_count = 0;
    // We houden bij welke getallen we al gezien hebben,
    // zodat we kunnen zien of we in een lus zitten
    let mut seen: HashSet<u128> = HashSet::new();
    while n != 1 {
        if seen.insert(n)
            && let Some(next) = next_collatz_safe(n)
        {
            n = next;
        } else {
            // We zitten in een lus of een overflow
            return None;
        }
        step_count += 1;
    }
    Some(step_count)
}

fn next_collatz_safe(n: u128) -> Option<u128> {
    if n.is_multiple_of(2) {
        Some(n / 2)
    } else {
        // De checked_ versies geven None terug bij een overflow
        n.checked_mul(3).and_then(|m| m.checked_add(1))
    }
}

fn main() {
    // We gaan achtereenvolgens alle positieve gehele getallen af
    // en we slaan voor ieder getal het aantal stappen op
    // Dit aantal stappen kan ook None zijn, zoals bij een overflow.
    let mut cache: HashMap<u128, Option<usize>> = HashMap::new();
    // We houden alle getallen bij waar we langs komen, zodat we
    // in een keer het aantal stappen voor al deze getallen kunnen
    // bepalen.
    let mut sequence = Vec::new();
    for mut n in 1..1_000 {
        // Als we het getal al in een eerdere reeks langs hebben zien
        // komen hoeven we niets te berekenen, want het aantal stappen
        // is al bekend.
        if cache.contains_key(&n) {
            continue;
        }
        let mut step_count = Some(0);
        while n != 1 {
            sequence.push(n);
            if let Some(maybe_count) = cache.get(&n) {
                step_count = match (step_count, maybe_count) {
                    // Als we gaandeweg een getal tegen komen dat al
                    // in de cache zit, dan weten we al hoeveel extra
                    // stappen we nog moeten zetten om op 1 uit te komen
                    (Some(inner), Some(count)) => Some(inner + count),
                    // Als dit aantal stappen onbekend is, dan is het
                    // aantal stappen voor alle getallen in de reeks onbekend
                    _ => None,
                };
                break;
            }
            if let Some(next) = next_collatz_safe(n) {
                n = next;
                step_count = step_count.map(|inner| inner + 1);
            } else {
                println!("Overflow: {n}");
                step_count = None;
                break;
            }
        }
        if let Some(step_count) = step_count {
            for (i, n) in sequence.iter().enumerate() {
                let step_count = step_count - i;
                cache.insert(*n, Some(step_count));
            }
        } else {
            for n in &sequence {
                cache.insert(*n, None);
            }
        }
        sequence.clear();
    }
    let mut c: Vec<_> = cache
        .iter()
        .filter_map(|(k, v)| v.map(|w| (k, w)))
        .filter(|(_, v)| *v > 500)
        .collect();
    c.sort();
    dbg!(c);
}

#[cfg(test)]
mod tests {
    use core::assert_eq;

    use super::*;

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
