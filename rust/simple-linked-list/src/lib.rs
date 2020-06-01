use std::iter::FromIterator;

#[derive(Clone, Debug, PartialEq)]
struct Node<T> {
    pub data: T,
    pub next: Option<Box<Node<T>>>,
}

pub struct SimpleLinkedList<T> {
    // Delete this field
    // dummy is needed to avoid unused parameter error during compilation
    head: Option<Box<Node<T>>>,
    size: usize,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            size: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn push(&mut self, _element: T) {
        self.size += 1;

        self.head = Some(
            Node {
                data: _element,
                next: self.head.take(),
            }
                .into(),
        )
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.head.is_none() {
            return None;
        }

        self.size -= 1;

        self.head.take()
            .and_then(|node| {
                self.head = node.next;
                Some(node.data)
            })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref()
            .map(|node| &node.data)
    }

    pub fn rev(self) -> SimpleLinkedList<T> {
        let mut new_list = SimpleLinkedList::new();
        let mut stack: Vec<T> = self.into();

        stack.drain(0..)
            .rev()
            .for_each(|item| new_list.push(item));

        new_list
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item=T>>(iter: I) -> Self {
        let mut linked_list = SimpleLinkedList::new();

        for item in iter {
            linked_list.push(item)
        }

        linked_list
    }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
// Given that implementation, converting to a vector is trivial:
//
// let vec: Vec<_> = simple_linked_list.into_iter().collect();
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(mut self) -> Vec<T> {
        let mut result = Vec::with_capacity(self.size);

        while let Some(item) = self.pop() {
            result.push(item);
        }

        result.reverse();
        result
    }
}
