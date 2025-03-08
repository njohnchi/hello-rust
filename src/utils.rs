fn is_lower(character: char) -> bool {
    if character >= 'a' && character <= 'z' {
        true
    } else {
        false
    }
}

fn is_upper(character: char) -> bool {
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

fn is_alpha(character: char) -> bool {
    if is_upper(character) || is_lower(character) {
        true
    } else {
        false
    }
}

fn is_digit(character: char) -> bool {
    if character >= '0' && character <= '9' {
        true
    } else {
        false
    }
}

fn mul(num1: i64, num2: i64) -> i64 {
    num1 * num2
}

#[cfg(test)]
mod test_utils {
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
    fn it_is_upper() {
        assert_eq!(is_upper('B'), true);
        assert_eq!(is_upper('H'), true);
        assert_eq!(is_upper('Q'), true);
        assert_eq!(is_upper('f'), false);
        assert_eq!(is_upper(','), false);
        assert_eq!(is_upper('8'), false);
    }

    #[test]
    fn it_is_alpha() {
        assert_eq!(is_alpha('B'), true);
        assert_eq!(is_alpha('H'), true);
        assert_eq!(is_alpha('q'), true);
        assert_eq!(is_alpha('f'), true);
        assert_eq!(is_alpha(','), false);
        assert_eq!(is_alpha('8'), false);
    }

    #[test]
    fn it_is_digit() {
        assert_eq!(is_digit('3'), true);
        assert_eq!(is_digit('0'), true);
        assert_eq!(is_digit('9'), true);
        assert_eq!(is_digit('A'), false);
        assert_eq!(is_digit('j'), false);
        assert_eq!(is_digit('['), false);
    }

    #[test]
    fn returns_abs() {
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
    fn abs_should_panic_for_min() {
        abs(i64::MIN);
    }

    #[test]
    fn it_multiples() {
        assert_eq!(mul(2, 16), 32);
        assert_eq!(mul(7, 0), 0);
        assert_eq!(mul(-5, 30), -150);
        assert_eq!(mul(-3, -8), 24);
    }
}