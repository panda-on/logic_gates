//! This is a logic gates simulation crate build to demostrate writing unit tests, integration test,
//! documentation tests.

#![doc(html_logo_url = "https://d30y9cdsu7xlg0.cloudfront.net/png/411962-200.png")]

/// Implements a bool `and` gate taking as input two bits and returning a bit as its
/// output
pub fn and(a: u8, b: u8) -> u8{
    match (a, b) {
        (1, 1) => 1,
        _ => 0
    }
}

/// Implements a bool `xor` gate taking as input two bits and returning a bit as its
/// output
pub fn xor(a: u8, b: u8) -> u8 {
    match (a, b) {
        (1, 0) | (0, 1) => 1,
        _ => 0
    }
}

#[cfg(test)]
mod tests {
    
    use crate::{and, xor};

    #[test]
    fn test_add() {
        assert_eq!(0, and(0, 0));
        assert_eq!(0, and(0, 1));
        assert_eq!(0, and(1, 0));
        assert_eq!(1, and(1, 1));
    }
    
    #[test]
    fn test_xor() {
        assert_eq!(0, xor(0, 0));
        assert_eq!(1, xor(0, 1));
        assert_eq!(1, xor(1, 0));
        assert_eq!(0, xor(1, 1));
    }
}