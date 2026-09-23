fn plus_een(getal: i32) -> i32 {
    getal + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_klein() {
        // TODO: Voeg een controle voor plus_een(1) toe.
    }

    #[test]
    fn test_groot() {
        // TODO: Voeg een controle voor plus_een(100) toe.
    }
}

fn main() {}