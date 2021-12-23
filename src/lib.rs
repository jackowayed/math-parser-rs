use std::collections::LinkedList;

pub fn rpn(m: &str) -> i32 {
    let mut stack: LinkedList<i32> = LinkedList::new();
    for token in m.split_ascii_whitespace() {
        stack.push_back(token.parse().unwrap());
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
}