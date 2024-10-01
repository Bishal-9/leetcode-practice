/*

7. Reverse Integer

https://leetcode.com/problems/reverse-integer/description/

*/

use std::io::stdin;

#[allow(unused)]
fn main() {
    let mut _input = String::new();
    stdin().read_line(&mut _input).unwrap();
    _input = _input.trim().to_string();
    let integer: i32 = _input.parse().unwrap();

    let _result = _reverse_integer(integer);
    println!("{}", _result);
}

fn _reverse_integer(mut x: i32) -> i32 {
    let mut _result: i32 = 0;
    let mut is_negetive = false;

    if x < 0 {
        is_negetive = true;
        x = x * -1;
    }

    while x > 0 {
        let r = _result.checked_mul(10);
        match r {
            Some(_r) => {
                _result = _result * 10
            },
            None => return 0
        }
        let r = _result.checked_add(x % 10);
        match r {
            Some(_r) => {
                _result = _result + (x % 10)
            },
            None => return 0
        }
        x = x / 10;
    }

    if is_negetive {
        _result = _result * -1;
    }

    _result
}

#[cfg(test)]
mod tests {

    use super::_reverse_integer;

    #[test]
    fn test_case_1() {
        let _result = _reverse_integer(123);
        assert_eq!(_result, 321);
    }

    #[test]
    fn test_case_2() {
        let _result = _reverse_integer(-123);
        assert_eq!(_result, -321);
    }

    #[test]
    fn test_case_3() {
        let _result = _reverse_integer(120);
        assert_eq!(_result, 21);
    }

    #[test]
    fn test_case_4() {
        let _result = _reverse_integer(1534236469);
        assert_eq!(_result, 0);
    }
}
