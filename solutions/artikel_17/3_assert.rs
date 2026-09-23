fn is_levend(gezondheid: i32) -> bool {
    gezondheid > 0
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn controleer_levend() {
        assert!(is_levend(10));
        assert!(!is_levend(0));
    }
}
