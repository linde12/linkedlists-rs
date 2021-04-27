use std::ptr;

#[derive(Debug)]
pub struct Queue<T> {
    head: Link<T>,
    tail: *mut Node<T>, // unsafe
    size: usize,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: ptr::null_mut(),
            size: 0,
        }
    }

    pub fn push(&mut self, data: T) {
        let mut next = Box::new(Node {
            data,
            next: None,
        });
        let raw_next: *mut Node<T> = &mut *next;

        if self.tail.is_null() {
            self.head = Some(next)
        } else {
            unsafe {
                (*self.tail).next = Some(next)
            }
        }

        // update tail
        self.tail = raw_next;

        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|val| {
            self.head = val.next;
            self.size -= 1;

            // if queue is empty, drop tail
            if self.head.is_none() {
                self.tail = ptr::null_mut();
            }

            val.data
        })
    }

    pub fn size(&self) -> usize {
        self.size
    }
}

#[derive(Debug)]
struct Node<T> {
    data: T,
    next: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn push_pop() {
        let mut s = Queue::new();
        s.push("foo");
        s.push("bar");
        s.push("baz");
        assert_eq!(s.size(), 3);
        let maybe_foo = s.pop();
        assert_eq!(maybe_foo, Some("foo"));
        let maybe_bar = s.pop();
        assert_eq!(maybe_bar, Some("bar"));
        let maybe_baz = s.pop();
        assert_eq!(maybe_baz, Some("baz"));
        let nothing = s.pop();
        assert_eq!(nothing, None);
        assert_eq!(s.size(), 0);
    }
}
