/*

2. Add Two Numbers

https://leetcode.com/problems/add-two-numbers/

*/

use std::io::stdin;

#[derive(Debug, Clone, PartialEq)]
struct ListNode {
    pub value: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(_value: i32) -> Self {
        ListNode {
            next: None,
            value: _value,
        }
    }
}

#[allow(unused)]
fn main() {
    let mut _input = String::new();
    stdin().read_line(&mut _input).unwrap();
    _input = _input.trim().to_string();

    _input.remove(0);
    _input.pop();

    let nums: Vec<i32> = _input.split(",").map(|i| i.parse().unwrap()).collect();
    let mut l1 = None;
    for i in nums.iter() {
        if l1.is_some() {
            let _new_item = ListNode {
                next: l1,
                value: *i,
            };
            l1 = Some(Box::new(_new_item));
        } else if l1.is_none() {
            l1 = Some(Box::new(ListNode::new(*i)));
        }
    }

    _input = String::new();
    stdin().read_line(&mut _input).unwrap();
    _input = _input.trim().to_string();

    _input.remove(0);
    _input.pop();

    let nums: Vec<i32> = _input.split(",").map(|i| i.parse().unwrap()).collect();
    let mut l2 = None;
    for i in nums.iter() {
        if l2.is_some() {
            let _new_item = ListNode {
                next: l2,
                value: *i,
            };
            l2 = Some(Box::new(_new_item));
        } else if l2.is_none() {
            l2 = Some(Box::new(ListNode::new(*i)));
        }
    }

    let _result = add_two_numbers(l1, l2);
    println!("{:?}", _result);
}

fn get_number(list: Option<Box<ListNode>>, mut _number: i32) -> i32 {
    let list = list.unwrap();
    let _value = &list.value;
    _number = (_number * 10_i32) + _value;
    if list.next.is_some() {
        _number = get_number(list.next, _number);
        _number
    } else {
        _number
    }
}

fn get_list(_number: i32) -> Option<Box<ListNode>> {
    let mut list: Option<Box<ListNode>> = None;
    for digit in _number.to_string().chars() {
        if list.clone().is_none() {
            let _value: i32 = digit.to_digit(10).unwrap() as i32;
            list = Some(Box::new(ListNode::new(_value)));
        } else {
            let _value: i32 = digit.to_digit(10).unwrap() as i32;
            let new_list = ListNode {
                next: list,
                value: _value,
            };
            list = Some(Box::new(new_list));
        }
    }

    list
}

fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let num_1 = get_number(l1, 0);
    let num_2 = get_number(l2, 0);
    let _result_num = num_1 + num_2;

    // println!("Number 1: {}", num_1);
    // println!("Number 2: {}", num_2);
    // println!("Result Number: {}", _result_num);

    let _result_list = get_list(_result_num);
    // println!("Result List: {:?}", _result_list);

    _result_list
}

#[cfg(test)]
mod tests {

    use super::{add_two_numbers, ListNode};

    #[test]
    fn test_case_1() {
        let nums: Vec<i32> = vec![2, 4, 3];
        let mut l1 = None;
        for i in nums.iter() {
            if l1.is_some() {
                let _new_item = ListNode {
                    next: l1,
                    value: *i,
                };
                l1 = Some(Box::new(_new_item));
            } else if l1.is_none() {
                l1 = Some(Box::new(ListNode::new(*i)));
            }
        }

        let nums: Vec<i32> = vec![5, 6, 4];
        let mut l2 = None;
        for i in nums.iter() {
            if l2.is_some() {
                let _new_item = ListNode {
                    next: l2,
                    value: *i,
                };
                l2 = Some(Box::new(_new_item));
            } else if l2.is_none() {
                l2 = Some(Box::new(ListNode::new(*i)));
            }
        }

        let _result = add_two_numbers(l1, l2);
        let expected_output = Some(Box::new(ListNode {
            value: 7,
            next: Some(Box::new(ListNode {
                value: 0,
                next: Some(Box::new(ListNode {
                    value: 8,
                    next: None,
                })),
            })),
        }));
        assert_eq!(_result, expected_output);
    }

    #[test]
    fn test_case_2() {
        let nums: Vec<i32> = vec![0];
        let mut l1 = None;
        for i in nums.iter() {
            if l1.is_some() {
                let _new_item = ListNode {
                    next: l1,
                    value: *i,
                };
                l1 = Some(Box::new(_new_item));
            } else if l1.is_none() {
                l1 = Some(Box::new(ListNode::new(*i)));
            }
        }

        let nums: Vec<i32> = vec![0];
        let mut l2 = None;
        for i in nums.iter() {
            if l2.is_some() {
                let _new_item = ListNode {
                    next: l2,
                    value: *i,
                };
                l2 = Some(Box::new(_new_item));
            } else if l2.is_none() {
                l2 = Some(Box::new(ListNode::new(*i)));
            }
        }

        let _result = add_two_numbers(l1, l2);
        let expected_output = Some(Box::new(ListNode {
            value: 0,
            next: None,
        }));
        assert_eq!(_result, expected_output);
    }

    #[test]
    fn test_case_3() {
        let nums: Vec<i32> = vec![9,9,9,9,9,9,9];
        let mut l1 = None;
        for i in nums.iter() {
            if l1.is_some() {
                let _new_item = ListNode {
                    next: l1,
                    value: *i,
                };
                l1 = Some(Box::new(_new_item));
            } else if l1.is_none() {
                l1 = Some(Box::new(ListNode::new(*i)));
            }
        }

        let nums: Vec<i32> = vec![9,9,9,9];
        let mut l2 = None;
        for i in nums.iter() {
            if l2.is_some() {
                let _new_item = ListNode {
                    next: l2,
                    value: *i,
                };
                l2 = Some(Box::new(_new_item));
            } else if l2.is_none() {
                l2 = Some(Box::new(ListNode::new(*i)));
            }
        }

        let _result = add_two_numbers(l1, l2);
        let expected_output = Some(Box::new(ListNode {
            value: 8,
            next: Some(Box::new(ListNode {
                value: 9,
                next: Some(Box::new(ListNode {
                    value: 9,
                    next: Some(Box::new(ListNode {
                        value: 9,
                        next: Some(Box::new(ListNode {
                            value: 0,
                            next: Some(Box::new(ListNode {
                                value: 0,
                                next: Some(Box::new(ListNode {
                                    value: 0,
                                    next: Some(Box::new(ListNode {
                                        value: 1,
                                        next: None
                                    }))
                                }))
                            }))
                        }))
                    })),
                })),
            })),
        }));
        assert_eq!(_result, expected_output);
    }

    #[test]
    fn test_case_4() {
        let nums: Vec<i32> = vec![2, 4, 9];
        let mut l1 = None;
        for i in nums.iter() {
            if l1.is_some() {
                let _new_item = ListNode {
                    next: l1,
                    value: *i,
                };
                l1 = Some(Box::new(_new_item));
            } else if l1.is_none() {
                l1 = Some(Box::new(ListNode::new(*i)));
            }
        }

        let nums: Vec<i32> = vec![5, 6, 4, 9];
        let mut l2 = None;
        for i in nums.iter() {
            if l2.is_some() {
                let _new_item = ListNode {
                    next: l2,
                    value: *i,
                };
                l2 = Some(Box::new(_new_item));
            } else if l2.is_none() {
                l2 = Some(Box::new(ListNode::new(*i)));
            }
        }

        let _result = add_two_numbers(l1, l2);
        let expected_output = Some(Box::new(ListNode {
            value: 7,
            next: Some(Box::new(ListNode {
                value: 0,
                next: Some(Box::new(ListNode {
                    value: 4,
                    next: Some(Box::new(ListNode {
                        value: 0,
                        next: Some(Box::new(ListNode {
                            value: 1,
                            next: None
                        }))
                    })),
                })),
            })),
        }));
        assert_eq!(_result, expected_output);
    }
}
