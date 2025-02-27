fn main() {
    is_valid("()".to_string());
}

pub fn is_valid(s: String) -> bool {
    match expr_zero_or_more(&s[..]) {
        Some(s) => s.is_empty(),
        None => false,
    }
}

fn expr_zero_or_more(s: &str) -> Option<&str> {
    // println!("expr_zero_or_more s={}", s);

    if s.is_empty() {
        return Some(s);
    }

    let mut t = s;
    while !t.is_empty() {
        match expr_parens(t) {
            Some(u) => t = u,
            None => return None,
        }
        // println!("expr_zero_or_more while t={}", t);
    }
    Some("")
}

fn expr_one_or_more(s: &str) -> Option<&str> {
    // println!("expr_one_or_more s={}", s);

    let mut t = s;
    while !t.is_empty() {
        // println!("expr_one_or_more while t={}", t);
        match expr_parens(t) {
            Some(u) => t = u,
            None => return Some(t),
        }
        // println!("expr_one_or_more while match t={}", t);
    }
    Some("")
}

fn expr_parens(s: &str) -> Option<&str> {
    // println!("expr_parens s={}", s);
    assert!(!s.is_empty(), "s must not be empty");

    let next = &s[0..1];
    match next {
        "(" => {
            // println!("expr_parens s={} match ( &s[1..]={}", s, &s[1..]);

            if s.len() == 1 {
                return None
            }

            let peek = &s[1..2];
            match peek {
                ")" => {
                    // println!("expr_parens s={} match ( peek ) &s[2..]={}", s, &s[1..]);
                    Some(&s[2..])
                },
                _ => {

                    let e = expr_one_or_more(&s[1..]);
                    // println!("expr_parens s={} match ( e={:?}", s, e);
                    match e {
                        Some(f) => {
                            if !f.is_empty() && &f[0..1] == ")" {
                                // println!("expr_parens s={} ( e={:?} return Some(&f[1..])={}", s, e, &f[1..]);
                                Some(&f[1..])
                            } else {
                                None
                            }
                        },
                        None => Some(s)
                    }

                }
            }
        },
        "{" => {
            // println!("expr_parens s={} match {{ &s[1..]={}", s, &s[1..]);

            if s.len() == 1 {
                return None
            }

            let peek = &s[1..2];
            match peek {
                "}" => {
                    // println!("expr_parens s={} match {{ peek }} &s[2..]={}", s, &s[1..]);
                    Some(&s[2..])
                },
                _ => {

                    let e = expr_one_or_more(&s[1..]);
                    // println!("expr_parens s={} match {{ e={:?}", s, e);
                    match e {
                        Some(f) => {
                            if !f.is_empty() && &f[0..1] == "}" {
                                // println!("expr_parens s={} {{ e={:?} return Some(&f[1..])={}", s, e, &f[1..]);
                                Some(&f[1..])
                            } else {
                                None
                            }
                        },
                        None => Some(s)
                    }

                }
            }
        },
        "[" => {
            // println!("expr_parens s={} match [ &s[1..]={}", s, &s[1..]);

            if s.len() == 1 {
                return None
            }

            let peek = &s[1..2];
            match peek {
                "]" => {
                    // println!("expr_parens s={} match [ peek ] &s[2..]={}", s, &s[1..]);
                    Some(&s[2..])
                },
                _ => {

                    let e = expr_one_or_more(&s[1..]);
                    // println!("expr_parens s={} match [ e={:?}", s, e);
                    match e {
                        Some(f) => {
                            if !f.is_empty() && &f[0..1] == "]" {
                                // println!("expr_parens s={} [ e={:?} return Some(&f[1..])={}", s, e, &f[1..]);
                                Some(&f[1..])
                            } else {
                                None
                            }
                        },
                        None => Some(s)
                    }

                }
            }
        },
        _ => None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "s must not be empty")]
    fn test_expr_empty_panics() {
        let input = "";
        expr_parens(input);
    }

    #[test]
    fn test_expr_pair_is_some() {
        let input = "()";
        let result = expr_parens(input);
        assert!(result.is_some());
    }

    #[test]
    fn test_expr_nested_is_some() {
        let input = "(())";
        let result = expr_parens(input);
        assert!(result.is_some());
    }

    #[test]
    fn test_expr_nesteded_is_some() {
        let input = "((()))";
        let result = expr_parens(input);
        assert!(result.is_some());
    }

    #[test]
    fn test_expr_single_is_none() {
        let input = "(";
        let result = expr_parens(input);
        assert!(result.is_none());
    }

    #[test]
    fn test_empty_is_some() {
        let input = "";
        let result = expr_zero_or_more(input); 
        assert!(result.is_some());
    }

    #[test]
    fn test_single_is_none() {
        let input = "(";
        let result = expr_zero_or_more(input);
        assert!(result.is_none(), "result should be none");
    }

    #[test]
    fn test_parens_is_some() {
        let input = "()";
        let result = expr_zero_or_more(input);
        assert!(result.is_some());
    }

    #[test]
    fn test_parens_pairs_is_some() {
        let input = "()()";
        let result = expr_zero_or_more(input);
        assert!(result.is_some());
    }

    #[test]
    fn test_expr_zero_or_more_3_parens_pairs_is_some() {
        let input = "()()()";
        let result = expr_zero_or_more(input);
        assert!(result.is_some());
    }

    #[test]
    fn test_parens_nested_is_some() {
        let input = "(())";
        let result = expr_zero_or_more(input);
        assert!(result.is_some());
    }

    #[test]
    fn test_parens_nesteded_is_some() {
        let input = "((()))";
        let result = expr_zero_or_more(input);
        assert!(result.is_some());
    }

    #[test]
    fn test_parens_extra_open_is_none() {
        let input = "()(";
        let result = expr_zero_or_more(input);
        assert!(result.is_none());
    }

    #[test]
    fn test_parens_pair_extra_open_is_none() {
        let input = "()()(";
        let result = expr_zero_or_more(input);
        assert!(result.is_none());
    }

    #[test]
    fn test_parens_nested_pair_extra_open_is_none() {
        let input = "(())(()())(";
        let result = expr_zero_or_more(input);
        assert!(result.is_none());
    }

    #[test]
    fn test_parens_complex_is_some() {
        let input = "(())(()())()";
        let result = expr_zero_or_more(input);
        assert!(result.is_some(), "result should be true");
    }

    #[test]
    fn test_parens_extra_closed_is_none() {
        let input = "())";
        let result = expr_zero_or_more(input);
        assert!(result.is_none(), "result should be false");
    }
}
