/*

3. Longest Substring Without Repeating Characters

https://leetcode.com/problems/longest-substring-without-repeating-characters/description/

*/

use std::{collections::HashSet, io::stdin};

#[allow(unused)]
fn main() {

    let mut _input = String::new();
    stdin().read_line(&mut _input).unwrap();
    _input = _input.trim().to_string();

    let _result = _calculate_largest_string(_input);
    println!("{}", _result);
}

fn _calculate_largest_string(_string: String) -> usize {

    let mut result = 0;
    let mut repeated_character = HashSet::new();

    for character in _string.chars() {

        if repeated_character.contains(&character) && result < repeated_character.len() {
            result = repeated_character.len();
            repeated_character = HashSet::new();
            repeated_character.insert(character);
        } else {
            repeated_character.insert(character);
        }
    }

    if repeated_character.len() > result {
        result = repeated_character.len()
    }

    result
}

#[cfg(test)]
mod tests {

    use super::_calculate_largest_string;

    #[test]
    fn test_case_1() {
        let _result = _calculate_largest_string(String::from("abcabcbb"));
        assert_eq!(_result, 3);
    }

    #[test]
    fn test_case_2() {
        let _result = _calculate_largest_string(String::from("bbbbb"));
        assert_eq!(_result, 1);
    }

    #[test]
    fn test_case_3() {
        let _result = _calculate_largest_string(String::from("pwwkew"));
        assert_eq!(_result, 3);
    }

    #[test]
    fn test_case_4() {
        let _result = _calculate_largest_string(String::from("aab"));
        assert_eq!(_result, 2);
    }
}
