
/*

10. Regular Expression Matching

https://leetcode.com/problems/regular-expression-matching/description/

*/

use std::io::stdin;

#[allow(unused)]
fn main() {
    let mut _input = String::new();
    stdin().read_line(&mut _input).unwrap();
    _input = _input.trim().to_string();
    let _string = _input;

    _input = String::new();
    stdin().read_line(&mut _input).unwrap();
    _input = _input.trim().to_string();
    let _pattern = _input;

    let result = _regular_pattern(_string, _pattern);
    println!("{}", result);
}

fn _regular_pattern(s: String, p: String) -> bool {
    let mut result = true;

    let string_characters: Vec<char> = s.chars().into_iter().collect();
    let pattern_characters: Vec<char> = p.chars().into_iter().collect();

    let mut s_length = string_characters.len() - 1;
    let mut p_length = pattern_characters.len() - 1;

    if !p.contains('*') && s_length != p_length {
        return false;
    }

    while s_length >= 0 && p_length >= 0 {

        let _p = pattern_characters[p_length];
        let _s = string_characters[s_length];
        if _p == '.' {
            p_length = p_length - 1;
            s_length = s_length - 1;
            continue;
        } else if _p == '*' {
            break;
        } else if _p != _s {
            result = false;
            break;
        } else if _p == _s {
            p_length = p_length - 1;
            s_length = s_length - 1;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::_regular_pattern;

    #[test]
    fn test_case_1() {
        let result = _regular_pattern(String::from("aa"), String::from("a"));
        assert_eq!(result, false);
    }

    #[test]
    fn test_case_2() {
        let result = _regular_pattern(String::from("aa"), String::from("a*"));
        assert_eq!(result, true);
    }

    #[test]
    fn test_case_3() {
        let result = _regular_pattern(String::from("ab"), String::from(".*"));
        assert_eq!(result, true);
    }

    #[test]
    fn test_case_4() {
        let result = _regular_pattern(String::from("aab"), String::from("c*a*b"));
        assert_eq!(result, true);
    }
}
