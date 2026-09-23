fn plus_een(getal: i32) -> i32 {
    getal + 1
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_klein() {
        assert_eq!(plus_een(1), 2);
    }

    #[test]
    fn test_groot() {
        assert_eq!(plus_een(100), 101);
    }
}
