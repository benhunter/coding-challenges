fn main() {
    expr_zero_or_more("()");
}

fn expr_zero_or_more(s: &str) -> Option<&str> {
    println!("expr_zero_or_more s={}", s);

    if s.len() == 0 {
        return Some(s);
    }

    let mut t = s;
    while t.len() > 0 {
        match expr_parens(t) {
            Some(u) => t = u,
            None => return None,
        }
        println!("expr_zero_or_more while t={}", t);
    }
    Some("")
}

fn expr_one_or_more(s: &str) -> Option<&str> {
    println!("expr_one_or_more s={}", s);

    let mut t = s;
    while t.len() > 0 {
        println!("expr_one_or_more while t={}", t);
        match expr_parens(t) {
            Some(u) => t = u,
            None => return Some(t),
        }
        println!("expr_one_or_more while match t={}", t);
    }
    Some("")
}

fn expr_parens(s: &str) -> Option<&str> {
    println!("expr_parens s={}", s);
    assert!(s.len() > 0);

    let next = &s[0..1];
    match next {
        "(" => {
            println!("expr_parens s={} match ( &s[1..]={}", s, &s[1..]);

            if s.len() == 1 {
                return None
            }

            let peek = &s[1..2];
            match peek {
                ")" => {
                    println!("expr_parens s={} match ( peek ) &s[2..]={}", s, &s[1..]);
                    Some(&s[2..])
                },
                _ => {

                    let e = expr_one_or_more(&s[1..]);
                    println!("expr_parens s={} match ( e={:?}", s, e);
                    match e {
                        Some(f) => {
                            if f.len() >= 1 && &f[0..1] == ")" {
                                println!("expr_parens s={} ( e={:?} return Some(&f[1..])={}", s, e, &f[1..]);
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
    #[should_panic(expected = "assertion failed: s.len() > 0")]
    fn test_expr_parens_empty_is_none() {
        let input = "";
        let result = expr_parens(input);
        assert!(result.is_none());
    }

    #[test]
    fn test_expr_parens_pair_is_some() {
        let input = "()";
        let result = expr_parens(input);
        assert!(result.is_some());
    }

    #[test]
    fn test_expr_parens_nested_is_some() {
        let input = "(())";
        let result = expr_parens(input);
        assert!(result.is_some());
    }

    #[test]
    fn test_expr_parens_nesteded_is_some() {
        let input = "((()))";
        let result = expr_parens(input);
        assert!(result.is_some());
    }

    #[test]
    fn test_expr_parens_single_is_none() {
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
        assert!(result.is_none());
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
        assert!(result.is_some());
    }

    #[test]
    fn test_parens_extra_closed_is_none() {
        let input = "())";
        let result = expr_zero_or_more(input);
        assert!(result.is_none());
    }
}
