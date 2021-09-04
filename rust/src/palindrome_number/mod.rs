/// https://leetcode.com/problems/palindrome-number/

#[allow(dead_code)]
pub fn is_palindrome(x: i32) -> bool {
    let x_str = x.to_string().chars().collect::<Vec<char>>();

    let mut i = 0 as usize;
    let mut j = x_str.len() - 1;
    let middle = x_str.len()/2;
    loop {
        if x_str[i] != x_str[j] {
            return false;
        }
        i += 1;
        j -= 1;
        if i >= middle {
            break;
        }
    }
    true
}

#[allow(dead_code)]
pub fn is_palindrome_2(x: i32) -> bool {
    if x < 0 || (x % 10 == 0 && x != 0) {
        return false;
    }

    let mut reversed = 0;
    let mut m_x = x;
    while m_x > reversed {
        reversed = reversed * 10 + m_x % 10;
        m_x /= 10;
        println!("{} {}", reversed, m_x);
    }
   
    // for odd and even numbers
    m_x == reversed || m_x == reversed/10
}

#[cfg(test)]
mod tests {
    #[test]
    fn is_palindrome_test() {
        assert!(super::is_palindrome(121));
        assert!(!super::is_palindrome(-121));
        assert!(!super::is_palindrome(10));
        assert!(!super::is_palindrome(-101));
        assert!(super::is_palindrome(1221));
    }

    #[test]
    fn is_palindrome_test_2() {
        assert!(super::is_palindrome_2(121));
        assert!(!super::is_palindrome_2(-121));
        assert!(!super::is_palindrome_2(10));
        assert!(!super::is_palindrome_2(-101));
        assert!(super::is_palindrome_2(1221));
    }
}
