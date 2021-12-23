use std::collections::LinkedList;

fn do_operation(token: &str, stack: &mut LinkedList<i32>) -> () {
    let right = stack.pop_back().unwrap();
    let left = stack.pop_back().unwrap();
    let result = match token {
        "+" => left + right,
        "-" => left - right,
        _ => panic!("unsupported operand")
    };
    stack.push_back(result);
}

pub fn rpn(m: &str) -> i32 {
    let mut stack: LinkedList<i32> = LinkedList::new();
    for token in m.split_ascii_whitespace() {
        match token {
            "+" | "-" => do_operation(token, &mut stack),
            _ => stack.push_back(token.parse().unwrap()),
        }
        if token == "+" {
        } else {
            
        }
    }
    assert_eq!(stack.len(), 1);
    *stack.front().unwrap()
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn rpn_trivial() {
        assert_eq!(3, rpn("3"))
    }

    #[test]
    fn basic_addition() {
        assert_eq!(5, rpn("2 3 +"))
    }

    #[test]
    fn basic_subtract() {
        assert_eq!(-1, rpn("2 3 -"))
    }
}