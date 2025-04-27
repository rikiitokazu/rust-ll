use std::mem;

pub struct List {
    head: Link,
}


// [question] type vs enum
// Box ensures heap allocation
type Link = Option<Box<Node>>;

struct Node {
    elem: i32,
    next: Link,
}

impl List {
    pub fn new() -> Self{
        // Link :: Empty refers to VARIANT of Link because it is an enum
        List { head: None }
    }
    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem: elem,
            next: mem::replace(&mut self.head, None),
        });

        self.head = Some(new_node);
    }
    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, None) {
            None => None,
            Some(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}

// Drop trait is built-in rust, but to avoid recursion, we can 
// impelemtn it ourselves for the List
// [question] traits vs variants 
impl Drop for List {
    fn drop(&mut self) {
        // mem::replcae returns self.head before replace
        let mut curr_node = mem::replace(&mut self.head, None);

        // while let because we are assigning, not checking equality
        while let Some(mut boxed) = curr_node {
            curr_node = mem::replace(&mut boxed.next, None);
        }
    }
}

