fn verdubbel(getal: i32) -> i32 {
    getal * 2
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verdubbel() {
        assert_eq!(verdubbel(5), 10);
    }
}
