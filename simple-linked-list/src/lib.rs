use std::iter::FromIterator;

struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList { head: None }
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn len(&self) -> usize {
        let mut sum = 0;
        let mut next = &self.head;

        while let Some(next_node) = next {
            next = &next_node.next;
            sum += 1;
        }

        sum
    }

    pub fn push(&mut self, _element: T) {
        let existing_head = self.head.take();
        self.head = Some(Box::new(Node {
            data: _element,
            next: existing_head,
        }));
    }

    pub fn pop(&mut self) -> Option<T> {
        let existing_head = self.head.take();

        if let Some(head) = existing_head {
            self.head = head.next;
            Some(head.data)
        } else {
            None
        }
    }

    pub fn peek(&self) -> Option<&T> {
        if let Some(head) = &self.head {
            Some(&head.data)
        } else {
            None
        }
    }

    pub fn rev(self) -> SimpleLinkedList<T> {
        let mut result = SimpleLinkedList::new();
        let mut next_item = self.head;

        while let Some(item) = next_item {
            result.push(item.data);
            next_item = item.next;
        }

        result
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(_iter: I) -> Self {
        let mut result = SimpleLinkedList::new();

        _iter.into_iter().for_each(|item| result.push(item));

        result
    }
}

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(self) -> Vec<T> {
        let mut result = Vec::new();
        let mut next_item = self.head;

        while let Some(item) = next_item {
            result.insert(0, item.data);
            next_item = item.next;
        }

        result
    }
}
