use std::collections::VecDeque;

pub struct IntoPeekable<I, T> where I: Iterator<Item=T>, T: Clone {
    iter: I,
    queue: VecDeque<T>
}

impl<I: Iterator<Item=T>, T: Clone> IntoPeekable<I, T> {
    pub fn new(iter: I) -> Self {
        Self {
            iter,
            queue: VecDeque::new(),
        }
    }

    pub fn peek(&mut self) -> Option<<Self as Iterator>::Item> {
        if self.queue.is_empty() {
            if let Some(next) = self.iter.next() {
                self.queue.push_back(next);
            }
        }
        self.queue.front().cloned()
    }
}

impl<I: Iterator<Item=T>, T: Clone> Iterator for IntoPeekable<I, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(item) = self.queue.pop_front() {
            Some(item)
        } else {
            self.iter.next()
        }
    }
}

pub trait NewIntoPeekable {
    type PeekItem;
    type Iterator;

    fn into_peekable(self) -> IntoPeekable<Self::Iterator, Self::PeekItem>
    where
        Self: Sized,
        Self::PeekItem: Clone,
        Self::Iterator: Iterator<Item=Self::PeekItem>;
}

impl<I: Iterator<Item=T>, T: Clone> NewIntoPeekable for I {
    type PeekItem = I::Item;
    type Iterator = I;

    fn into_peekable(self) -> IntoPeekable<Self::Iterator, Self::PeekItem> {
        IntoPeekable::new(self)
    }
}
