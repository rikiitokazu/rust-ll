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
