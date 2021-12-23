use std::collections::LinkedList;

pub fn rpn(m: &str) -> i32 {
    let mut stack: LinkedList<i32> = LinkedList::new();
    for token in m.split_ascii_whitespace() {
        if token == "+" {
            let right = stack.pop_back().unwrap();
            let left = stack.pop_back().unwrap();
            stack.push_back(left + right);
        } else {
            stack.push_back(token.parse().unwrap());
        }
    }
    assert_eq!(stack.len(), 1);
    return *stack.front().unwrap();
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
}