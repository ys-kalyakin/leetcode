/// https://leetcode.com/problems/reverse-integer/

#[allow(dead_code)]
pub fn reverse(x: i32) -> i32 {
    let mut value = x;
    let mut reversed = 0;

    while value != 0 {
        let pop = value % 10;
        value /= 10;
        if reversed > i32::MAX / 10 || (reversed == i32::MAX / 10 && pop > 7) {
            return 0;
        }
        if reversed < i32::MIN / 10 || (reversed == i32::MIN / 10 && pop < -8) {
            return 0;
        }
        reversed = reversed * 10 + pop;
    }
    reversed
}

#[cfg(test)]
mod tests {
    #[test]
    fn reverse_integer_test() {
        assert_eq!(321, super::reverse(123));
        assert_eq!(-321, super::reverse(-123));
        assert_eq!(21, super::reverse(120));
        assert_eq!(0, super::reverse(0));
    }
}
