use crate::magic::Magic;

pub fn add(a: u32, b: u32) -> u32 {
    a + b
}

pub fn add_if_not_magic(a: u32, b: u32) -> Option<u32> {
    if a.is_magic() || b.is_magic() {
        None
    } else {
        Some(add(a, b))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 2), 4);
        assert_ne!(add(2, 2), 5);
    }

    #[test]
    fn test_magic_number() {
        assert!(42.is_magic());
        assert!(!41.is_magic());
    }

    #[test]
    fn test_add_magic_number() {
        match add_if_not_magic(2, 2) {
            None => assert!(false),
            Some(sum) => assert_eq!(sum, 4)
        }
    }
}