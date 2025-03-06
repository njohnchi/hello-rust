fn is_lower(character: char) -> bool {
    if character >= 'a' && character <= 'z' {
        true
    } else {
        false
    }
}

fn is_alpha(character: char) -> bool {
    if character >= 'A' && character <= 'Z' {
        true
    } else {
        false
    }
}

fn abs(number: i64) -> i64 {
    if number < 0 {
        -number
    } else {
        number
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_is_lower() {
        assert_eq!(is_lower('b'), true);
        assert_eq!(is_lower('o'), true);
        assert_eq!(is_lower('j'), true);
        assert_eq!(is_lower('O'), false);
        assert_eq!(is_lower('.'), false);
        assert_eq!(is_lower('3'), false);
    }

    #[test]
    fn it_is_alpha() {
        assert_eq!(is_alpha('B'), true);
        assert_eq!(is_alpha('H'), true);
        assert_eq!(is_alpha('Q'), true);
        assert_eq!(is_alpha('f'), false);
        assert_eq!(is_alpha(','), false);
        assert_eq!(is_alpha('8'), false);
    }

    #[test]
    fn test_abs() {
        assert_eq!(abs(0), 0);
        assert_eq!(abs(23), 23);
        assert_eq!(abs(-97), 97);
        assert_eq!(abs(789), 789);
        assert_eq!(abs(-4567), 4567);
        assert_ne!(abs(-378), -378);
        assert_eq!(abs(i64::MAX), i64::MAX);
    }

    #[test]
    #[should_panic]
    fn test_abs_min() {
        abs(i64::MIN);
    }
}