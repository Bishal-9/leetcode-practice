/*

8. String to Integer (atoi)

https://leetcode.com/problems/string-to-integer-atoi/description/

*/

use std::io::stdin;

#[allow(unused)]
fn main() {
    let mut _input = String::new();
    stdin().read_line(&mut _input).unwrap();
    _input = _input.trim().to_string();

    let result = _atoi(_input);
    println!("{}", result);
}

fn _atoi(_input: String) -> i32 {
    let mut result: i32 = 0;
    let mut is_negative = false;

    for (index, character) in _input.chars().enumerate() {
        if index == 0 && character == '-' {
            is_negative = true;
            continue;
        } else if index == 0 && character == '+' {
            continue;
        }

        match character {
            '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                let digit = (character.to_digit(10).unwrap()) as i32;
                match result.checked_mul(10) {
                    Some(_result) => match _result.checked_add(digit) {
                        Some(_) => {
                            result = (result * 10) + (character.to_digit(10).unwrap()) as i32;
                        }
                        None => {
                            if is_negative {
                                return i32::MIN;
                            } else {
                                return i32::MAX;
                            }
                        }
                    },
                    None => {
                        if is_negative {
                            return i32::MIN;
                        } else {
                            return i32::MAX;
                        }
                    }
                }
            }
            '0' => {
                if result > 0 {
                    let digit = (character.to_digit(10).unwrap()) as i32;
                    match result.checked_mul(10) {
                        Some(_result) => match _result.checked_add(digit) {
                            Some(_) => {
                                result = (result * 10) + (character.to_digit(10).unwrap()) as i32;
                            }
                            None => {
                                if is_negative {
                                    return i32::MIN;
                                } else {
                                    return i32::MAX;
                                }
                            }
                        },
                        None => {
                            if is_negative {
                                return i32::MIN;
                            } else {
                                return i32::MAX;
                            }
                        }
                    }
                } else {
                    continue;
                }
            }
            _ => {
                break;
            }
        }
    }

    if is_negative {
        result = result * -1;
    }

    result
}

#[cfg(test)]
mod tests {

    use super::_atoi;

    #[test]
    fn test_case_1() {
        let result = _atoi(String::from("42"));
        assert_eq!(result, 42);
    }

    #[test]
    fn test_case_2() {
        let result = _atoi(String::from("-042"));
        assert_eq!(result, -42);
    }

    #[test]
    fn test_case_3() {
        let result = _atoi(String::from("1337c0d3"));
        assert_eq!(result, 1337);
    }

    #[test]
    fn test_case_4() {
        let result = _atoi(String::from("0-1"));
        assert_eq!(result, 0);
    }

    #[test]
    fn test_case_5() {
        let result = _atoi(String::from("words and 987"));
        assert_eq!(result, 0);
    }

    #[test]
    fn test_case_6() {
        let result = _atoi(String::from("-91283472332"));
        assert_eq!(result, -2147483648);
    }

    #[test]
    fn test_case_7() {
        let result = _atoi(String::from("+1"));
        assert_eq!(result, 1);
    }

    #[test]
    fn test_case_8() {
        let result = _atoi(String::from("21474836460"));
        assert_eq!(result, 2147483647);
    }

    #[test]
    fn test_case_9() {
        let result = _atoi(String::from("2147483648"));
        assert_eq!(result, 2147483647);
    }
}
