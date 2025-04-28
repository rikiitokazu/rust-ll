use std::mem;

pub struct List<T> {
    head: Link<T>,
}

//tuple struct, so hence why self.0 referes to List<T>
pub struct IntoIter<T>(List<T>);
pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}


// [question] type vs enum
// Box ensures heap allocation
type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self{
        // Link :: Empty refers to VARIANT of Link because it is an enum
        List { head: None }
    }
    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem: elem,
            next: self.head.take(),
        });

        self.head = Some(new_node);
    }
    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }
    // Cannot transfer ownership on a mutable reference
    // i.e. set a variable equal to a dereferenced mutable reference
    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| {
            &node.elem
        })
    }
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn iter<'a>(&'a self) -> Iter<'a, T> {
        Iter { next: self.head.as_deref() }
    }
}

// Drop trait is built-in rust 
// implementing to avoid recursive blow up 
// [question] traits vs variants 
impl<T> Drop for List<T> {
    fn drop(&mut self) {
        // mem::replcae returns self.head before replace
        let mut curr_node = mem::replace(&mut self.head, None);

        // while let because we are assigning, not checking equality
        while let Some(mut boxed) = curr_node {
            curr_node = boxed.next.take();
        }
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}

#[cfg(test)]
mod test_ok {
    use super::List;
    #[test]
    fn tests() {
        let mut list = List::new();
        assert_eq!(list.peek(), None);
        list.push(1);
        assert_eq!(list.peek(), Some(&1));
        list.push(2);
        
        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(2)); 
        assert_eq!(iter.next(), Some(1)); 



    }
}
