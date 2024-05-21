pub struct LinkedList<T> {
    head: Link<T>,
}

struct Node<T> {
    element: T,
    next: Link<T>
}

type Link<T> = Option<Box<Node<T>>>;

impl<T> LinkedList<T> {
    fn new() -> Self {
        Self{head: None}
    }

    fn push(&mut self, element:T) {
        let old_head = self.head.take();
        let new_head = Box::new(Node {
            element,
            next: old_head,
        });

        self.head = Some(new_head);
    }

    fn pop(&mut self) -> Option<T> {
        let current_head = self.head.take();
        if let Some(n) = current_head {
            self.head = n.next;
            return Some(n.element);
        }

        None
    }

    fn peek(&self) -> Option<&T> {
        match &self.head {
            Some(n) => Some(&n.element),
            None => None
        }
    }

    fn iterator(&self) -> LinkedListIterattor<T> {
        LinkedListIterattor{
            next: self.head.as_deref()
        }
    }
}

pub struct LinkedListIterattor<'a, T> {
    next: Option<&'a Node<T>>
}

impl<'a, T> Iterator for LinkedListIterattor<'a, T>{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next {
            Some(n) => {
                self.next = n.next.as_deref();
                Some(&n.element)
            }
            None=> None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_pop() {
        let mut ll = LinkedList::new();
        ll.push(1);
        ll.push(2);
        ll.push(3);

        assert_eq!(ll.pop(), Some(3));
        assert_eq!(ll.pop(), Some(2));
        assert_eq!(ll.pop(), Some(1));
        assert_eq!(ll.pop(), None);
    }

    #[test]
    fn test_peek() {
        let mut ll = LinkedList::new();
        assert_eq!(ll.peek(), None);

        ll.push(1);
        assert_eq!(ll.peek(), Some(&1));

        ll.push(2);
        assert_eq!(ll.peek(), Some(&2));
    }

    #[test]
    fn test_iterator() {
        let mut ll = LinkedList::new();
        ll.push(1);
        ll.push(2);
        ll.push(3);

        let mut iter = ll.iterator();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);
    }
}
