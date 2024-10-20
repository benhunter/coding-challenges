mod add_two_numbers;


fn main() {
    let result = add_two_numbers::Solution::add_two_numbers(None, None);
    println!("{:?}", result)
}

#[cfg(test)]
mod tests {
    use crate::add_two_numbers::{self, ListNode};

    #[test]
    fn test_example1() {
        // * Input: l1 = [2,4,3], l2 = [5,6,4]
        // * Output: [7,0,8]
        // * Explanation: 342 + 465 = 807.
        let l1 = Some(
            Box::new(
                add_two_numbers::ListNode { val: 2, next:
                    Some(
                        Box::new(
                            add_two_numbers::ListNode { val: 4, next:
                                Some(Box::new(ListNode::new(3)))
                            }
                        )
                    )
                }
            )
        );
        let l2 = Some(
            Box::new(
                add_two_numbers::ListNode { val: 5, next:
                    Some(
                        Box::new(
                            add_two_numbers::ListNode { val: 6, next:
                                Some(Box::new(ListNode::new(4)))
                            }
                        )
                    )
                }
            )
        );
        let expected = Some(
            Box::new(
                add_two_numbers::ListNode{ val: 7, next:
                    Some(
                        Box::new(
                            add_two_numbers::ListNode { val: 0, next: Some(Box::new(ListNode::new(8))) }
                        )
                    )
                }
            )
        );

        let result = add_two_numbers::Solution::add_two_numbers(l1, l2);

        assert_eq!(expected, result);
        assert_eq!(expected.unwrap().val, result.unwrap().val);
    }
}
