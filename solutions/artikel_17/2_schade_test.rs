fn schade(gezondheid: i32, hoeveelheid: i32) -> i32 {
    gezondheid - hoeveelheid
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn schade_werkt() {
        assert_eq!(schade(100, 25), 75);
    }
}
