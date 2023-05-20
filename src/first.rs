use std::mem;

struct Node {
    elm: i32,
    next: Link,
}

enum Link {
    Empty,
    More(Box<Node>),
}

pub struct List {
    head: Link,
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.elm)
            }
        }
    }
    
    pub fn push(&mut self, elm: i32) {
        let new_node = Box::new(Node {
            elm: elm,
            next: mem::replace(&mut self.head, Link::Empty),
        });

        self.head = Link::More(new_node);
    }
}

#[cfg(test)]
mod test {
    use crate::first::List;
    #[test]
    fn basics() {
        let mut list = List::new();

        assert_eq!(list.pop(), None);

        // populate
        list.push(1);
        list.push(2);
        list.push(3);

        // pop
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));
        
        // push more
        list.push(4);
        list.push(5);
        
        // pop
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));


        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}
