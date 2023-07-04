use std::iter::{StepBy, Take};
use std::os::unix::raw::time_t;

pub struct GroupStepBy<I> {
    iter: I,
    step_size: usize,
    group_size: usize,
}

impl<I> GroupStepBy<I> {
    pub(crate) fn new(iter: I, step_size: usize, group_size: usize) -> Self {
        Self {
            iter,
            step_size,
            group_size,
        }
    }
}

impl<I, T> T for GroupStepBy<I> where I: Iterator, T: Iterator<Item = Iterator<Item = I::Item>> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.skip(self.step_size);
        Some(self.iter.take(self.group_size))
    }
}

pub trait GSBy<I> {
    fn group_step_by(self, step_size: usize, group_size: usize) -> GroupStepBy<I>;
}

impl<I: Iterator> GSBy<I> for I {
    fn group_step_by(self, step_size: usize, group_size: usize) -> GroupStepBy<I> {
        GroupStepBy::new(self, step_size, group_size)
    }
}