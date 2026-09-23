fn is_levend(gezondheid: i32) -> bool {
    gezondheid > 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn controleer_levend() {
        // TODO: Gebruik assert! en assert!(!...).
        assert!(!is_levend(10));
    }
}

fn main() {}