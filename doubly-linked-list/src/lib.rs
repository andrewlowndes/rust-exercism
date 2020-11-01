use std::marker::PhantomData;
use std::ptr;

mod pre_implemented;

pub struct Node<T> {
    value: T,
    next: *const Node<T>,
    prev: *const Node<T>,
}

pub struct LinkedList<T> {
    num_items: usize,
    head: *const Node<T>,
    tail: *const Node<T>,
}

pub struct Cursor<T> {
    list: *const LinkedList<T>,
    node: *const Node<T>,
}

pub struct Iter<'a, T: 'a> {
    node: *const Node<T>,
    _phantom: PhantomData<&'a T>,
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Node<T> {
    pub fn new(value: T) -> Self {
        Node {
            value,
            next: ptr::null_mut(),
            prev: ptr::null_mut(),
        }
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {
            num_items: 0,
            head: ptr::null_mut(),
            tail: ptr::null_mut(),
        }
    }

    pub fn len(&self) -> usize {
        self.num_items
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_null()
    }

    pub fn cursor_front(&mut self) -> Cursor<T> {
        Cursor {
            list: self,
            node: self.head as *mut Node<T>,
        }
    }

    pub fn cursor_back(&mut self) -> Cursor<T> {
        Cursor {
            list: self,
            node: self.tail as *mut Node<T>,
        }
    }

    pub fn iter(&self) -> Iter<T> {
        Iter {
            node: self.head,
            _phantom: PhantomData,
        }
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        unsafe {
            let mut next = self.head;

            while !next.is_null() {
                let node = Box::from_raw(next as *mut Node<T>);
                next = node.next;

                drop(node);
            }
        }
    }
}

unsafe impl<T: Send> Send for LinkedList<T> {}
unsafe impl<T: Sync> Sync for LinkedList<T> {}

impl<T> Cursor<T> {
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        unsafe {
            if !self.node.is_null() {
                Some(&mut (*(self.node as *mut Node<T>)).value)
            } else {
                None
            }
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> Option<&mut T> {
        unsafe {
            if !self.node.is_null() {
                self.node = (*self.node).next;
                self.peek_mut()
            } else {
                None
            }
        }
    }

    pub fn prev(&mut self) -> Option<&mut T> {
        unsafe {
            if !self.node.is_null() {
                self.node = (*self.node).prev;
                self.peek_mut()
            } else {
                None
            }
        }
    }

    pub fn take(&mut self) -> Option<T> {
        unsafe {
            if self.node.is_null() {
                return None;
            }

            let list = &mut *(self.list as *mut LinkedList<T>);
            let node = Box::from_raw(self.node as *mut Node<T>);

            if !node.next.is_null() {
                let next_node = &mut *(node.next as *mut Node<T>);

                if !node.prev.is_null() {
                    let prev_node = &mut *(node.prev as *mut Node<T>);

                    next_node.prev = node.prev;
                    prev_node.next = node.next;
                } else {
                    list.head = node.next;
                    next_node.prev = ptr::null_mut();
                }

                self.node = node.next;
            } else if !node.prev.is_null() {
                let prev_node = &mut *(node.prev as *mut Node<T>);

                prev_node.next = ptr::null_mut();
                list.tail = node.prev;
                self.node = node.prev;
            } else {
                list.head = ptr::null_mut();
                list.tail = ptr::null_mut();
                self.node = ptr::null_mut();
            }

            list.num_items -= 1;

            Some(node.value)
        }
    }

    pub fn insert_after(&mut self, _element: T) {
        unsafe {
            let mut new_node = Box::new(Node::new(_element));
            let new_node_pointer: *mut _;

            let list = &mut *(self.list as *mut LinkedList<T>);

            list.num_items += 1;

            if self.node.is_null() {
                new_node_pointer = Box::into_raw(new_node);
                list.head = new_node_pointer;
                list.tail = new_node_pointer;
                return;
            }

            let node = &mut *(self.node as *mut Node<T>);
            new_node.prev = self.node;

            if !node.next.is_null() {
                let next_node = &mut *(node.next as *mut Node<T>);
                new_node.next = node.next;
                new_node_pointer = Box::into_raw(new_node);
                next_node.prev = new_node_pointer;
            } else {
                new_node_pointer = Box::into_raw(new_node);
                list.tail = new_node_pointer;
            }

            node.next = new_node_pointer;
        }
    }

    pub fn insert_before(&mut self, _element: T) {
        unsafe {
            let mut new_node = Box::new(Node::new(_element));
            let new_node_pointer: *mut _;

            let list = &mut *(self.list as *mut LinkedList<T>);

            list.num_items += 1;

            if self.node.is_null() {
                new_node_pointer = Box::into_raw(new_node);
                list.head = new_node_pointer;
                list.tail = new_node_pointer;
                return;
            }

            let node = &mut *(self.node as *mut Node<T>);
            new_node.next = self.node;

            if !node.prev.is_null() {
                let prev_node = &mut *(node.prev as *mut Node<T>);
                new_node.prev = node.prev;
                new_node_pointer = Box::into_raw(new_node);
                prev_node.next = new_node_pointer;
            } else {
                new_node_pointer = Box::into_raw(new_node);
                list.head = new_node_pointer;
            }

            node.prev = new_node_pointer;
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        unsafe {
            if !self.node.is_null() {
                let node = &*self.node;
                self.node = node.next;
                Some(&node.value)
            } else {
                None
            }
        }
    }
}
