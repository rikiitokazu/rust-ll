use std::mem;

pub struct List {
    head: Link,
}

enum Link {
    Empty,
    More(Box<Node>),
}

struct Node {
    elem: i32,
    next: Link,
}

impl List {
    pub fn new() -> Self{
        // Link :: Empty refers to VARIANT of Link because it is an enum
        List { head: Link::Empty }
    }
    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem: elem,
            next: mem::replace(&mut self.head, Link::Empty),
        });

        self.head = Link::More(new_node);
    }
    pub fn pop(&mut self) -> Option<i32> {
        let res;
        match mem::replace(&mut self.head, Link:: Empty) {
            Link::Empty => {
                res = None
            }
            Link::More(node) => {
                res = Some(node.elem);
                self.head = node.next;
            }
        };
        res
    }
}

// Drop trait is built-in rust, but to avoid recursion, we can 
// impelemtn it ourselves for the List
// [question] traits vs variants 
impl Drop for List {
    fn drop(&mut self) {
        // mem::replcae returns self.head before replace
        let mut curr_node = mem::replace(&mut self.head, Link::Empty);

        // while let because we are assigning, not checking equality
        while let Link::More(mut boxed) = curr_node {
            curr_node = mem::replace(&mut boxed.next, Link::Empty);
        }
    }
}

// Tests  
// only compiled when cargo test
#[cfg(test)]
mod test {
    use super::List;
    #[test]
    fn basics() {
        let mut list = List::new();
        assert_eq!(list.pop(), None);

        list.push(1);
        list.push(2);
        
        assert_eq!(list.pop(), Some(2)); 
    }
}
