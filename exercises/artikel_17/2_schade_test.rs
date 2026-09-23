fn schade(gezondheid: i32, hoeveelheid: i32) -> i32 {
    gezondheid - hoeveelheid
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn schade_werkt() {
        // TODO: Verwacht 75.
        assert_eq!(schade(100, 25), 0);
    }
}

fn main() {}