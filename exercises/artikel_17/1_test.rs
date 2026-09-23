fn verdubbel(getal: i32) -> i32 {
    getal * 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verdubbel() {
        // TODO: Controleer verdubbel(5).
        assert_eq!(verdubbel(5), 0);
    }
}

fn main() {}