pub struct IntoIter<T>(Stack<T>);
pub struct Iter<'a, T>(Option<&'a Node<T>>);
pub struct IterMut<'a, T>(Option<&'a mut Node<T>>);

#[derive(Debug)]
pub struct Stack<T> {
    head: Link<T>,
    size: usize,
}

#[derive(Debug)]
struct Node<T> {
    data: T,
    next: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

impl<T> Stack<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            size: 0,
        }
    }

    pub fn push(&mut self, data: T) {
        let next = Some(Box::new(Node {
            data,
            next: self.head.take(),
        }));
        self.head = next;
        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|val| {
            self.head = val.next;
            self.size -= 1;
            val.data
        })
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter(self.head.as_deref())
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        // IterMut(self.head.take())
        IterMut(self.head.as_deref_mut())
    }
}

impl<T> IntoIterator for Stack<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.into_iter()
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
        self.0.map(|node| {
            self.0 = node.next.as_deref();
            &node.data
        })
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.take().map(|node| {
            self.0 = node.next.as_deref_mut();
            &mut node.data
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn push_pop() {
        let mut s = Stack::new();
        s.push("foo");
        s.push("bar");
        s.push("baz");
        assert_eq!(s.size(), 3);
        let maybe_baz = s.pop();
        assert_eq!(maybe_baz, Some("baz"));
        let maybe_bar = s.pop();
        assert_eq!(maybe_bar, Some("bar"));
        let maybe_foo = s.pop();
        assert_eq!(maybe_foo, Some("foo"));
        let nothing = s.pop();
        assert_eq!(nothing, None);
        assert_eq!(s.size(), 0);
    }

    #[test]
    fn iter() {
        let mut s = Stack::new();
        s.push("foo");
        s.push("bar");
        s.push("baz");
        assert_eq!(s.size(), 3);
        let mut it = s.iter();
        assert_eq!(it.next(), Some(&"baz"));
        assert_eq!(it.next(), Some(&"bar"));
        assert_eq!(it.next(), Some(&"foo"));
        assert_eq!(it.next(), None);
        s.push("hello");
        s.push("world");
        let res: Vec<String> = s.iter().filter(|s| s.len() > 3).map(|s| s.to_ascii_uppercase()).collect();
        assert_eq!(res, vec!["WORLD", "HELLO"]);
    }

    #[test]
    fn iter_mut() {
        let mut s = Stack::new();
        s.push("foo");
        s.push("bar");
        s.push("baz");
        assert_eq!(s.size(), 3);
        let mut it = s.iter_mut();
        assert_eq!(it.next(), Some(&mut "baz"));
        assert_eq!(it.next(), Some(&mut "bar"));
        assert_eq!(it.next(), Some(&mut "foo"));
        assert_eq!(it.next(), None);
        assert_eq!(s.size(), 3);
        s.push("topkek");
        let mut it = s.iter_mut();
        assert_eq!(it.next(), Some(&mut "topkek"));
        assert_eq!(s.size(), 4);

        // mutate
        for item in s.iter_mut() {
            *item = "hello";
        }
        assert_eq!(s.pop(), Some("hello"));
        assert_eq!(s.pop(), Some("hello"));
        assert_eq!(s.pop(), Some("hello"));
        assert_eq!(s.pop(), Some("hello"));
        assert_eq!(s.pop(), None);
    }

    #[test]
    fn intoiterator() {
        let mut s = Stack::new();
        s.push("foo");
        s.push("bar");
        s.push("baz");
        let mut v = vec![];
        for x in s {
            v.push(x);
        }
        assert_eq!(v, vec!["baz", "bar", "foo"]);
    }
}
