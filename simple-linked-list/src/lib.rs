pub struct SimpleLinkedList<T> {
   element: Option<T>,
   next: Option<Box<SimpleLinkedList<T>>>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self {
            element: None,
            next: None,
        }
    }

    // You may be wondering why it's necessary to have is_empty()
    // when it can easily be determined from len().
    // It's good custom to have both because len() can be expensive for some types,
    // whereas is_empty() is almost always cheap.
    // (Also ask yourself whether len() is expensive for SimpleLinkedList)
    pub fn is_empty(&self) -> bool {
        self.element.is_none()
    }

    pub fn len(&self) -> usize {
        if self.is_empty() {
            0
        } else {
            1 + self.next.as_ref().map_or(0, |next| next.len())
        }
    }

    pub fn push(&mut self, _element: T) {
     // Create new element and make it the new head of the list, with the old head as its next element
        let new_node = SimpleLinkedList {
            element: Some(_element),
            next: Some(Box::new(std::mem::replace(self, SimpleLinkedList::new()))),
        };
        *self = new_node;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            let element = self.element.take();
            if let Some(next) = self.next.take() {
                *self = *next;
            }
            element
        }
    }

    pub fn peek(&self) -> Option<&T> {
        self.element.as_ref()
    }

    #[must_use]
    pub fn rev(self) -> SimpleLinkedList<T> {
        let mut reversed_list = SimpleLinkedList::new();
        let mut current_list = self;

        while let Some(element) = current_list.element {
            reversed_list.push(element);
            if let Some(next) = current_list.next {
                current_list = *next;
            } else {
                break;
            }
        }

        reversed_list
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut list = SimpleLinkedList::new();
        for item in iter {
            list.push(item);
        }
        list
    }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.
//
// Please note that the "front" of the linked list should correspond to the "back"
// of the vector as far as the tests are concerned.

impl<T> From<SimpleLinkedList<T>> for Vec<T> {
    fn from(mut linked_list: SimpleLinkedList<T>) -> Vec<T> {
        let mut vec = Vec::new();
        while let Some(element) = linked_list.pop() {
            vec.push(element);
        }
        vec.reverse();
        vec
    }
}
