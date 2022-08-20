use crate::collections::{Collection, IntoIterable};

pub struct Queue<T>
    where T: Clone,
{
    queue: Vec<T>,
}

impl<T> Queue<T>
    where T: Clone,
{
    pub fn new() -> Self {
        Self { queue: Vec::new() }
    }

    pub fn from_vec(items: Vec<T>) -> Self {
        Self { queue: items }
    }

    pub fn enqueue(&mut self, item: T) {
        self.queue.push(item);
    }

    pub fn dequeue(&mut self) -> Option<T> {
        if !self.is_empty() {
            Some(self.queue.remove(0))
        } else {
            None
        }
    }

    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    pub fn peek(&self) -> Option<&T> {
        self.queue.first()
    }

    pub fn len(&self) -> usize {
        self.queue.len()
    }
}

impl<T: Clone> Default for Queue<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Clone> Collection for Queue<T> {
    type Item = T;
}

impl<T: Clone> IntoIterable for Queue<T> {
    type Item = T;
    type IntoIter = QueueIntoIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        QueueIntoIterator::from_queue(self)
    }
}

pub struct QueueIntoIterator<T>
    where T: Clone,
{
    queue: Queue<T>,
}

impl<T: Clone> QueueIntoIterator<T>
{
    pub fn from_queue(queue: Queue<T>) -> Self {
        Self { queue }
    }
}

impl<T: Clone> Iterator for QueueIntoIterator<T>
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.queue.dequeue()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_int_queue() -> Queue<i32> {
        let mut ints = Queue::<i32>::new();
        ints.enqueue(0);
        ints.enqueue(1);
        ints.enqueue(2);

        ints
    }

    #[test]
    fn new_works() {
        let ints = Queue::<i32>::new();
        let floats = Queue::<f32>::new();

        assert_eq!(ints.len(), 0);
        assert_eq!(floats.len(), 0);

        assert!(ints.is_empty());
        assert!(floats.is_empty());
    }

    #[test]
    fn from_vec_works() {
        let ints = Queue::from_vec(vec![0, 1, 2]);

        assert_eq!(ints.len(), 3);
        assert!(!ints.is_empty());

        assert_eq!(ints.queue.first(), Some(&0));
        assert_eq!(ints.queue.get(1), Some(&1));
        assert_eq!(ints.queue.get(2), Some(&2));
    }

    #[test]
    fn queueing_works() {
        let ints = make_int_queue();

        assert_eq!(ints.len(), 3);
        assert!(!ints.is_empty());

        assert_eq!(ints.queue.first(), Some(&0));
        assert_eq!(ints.queue.get(1), Some(&1));
        assert_eq!(ints.queue.get(2), Some(&2));
    }

    #[test]
    fn dequeueing_works() {
        let mut ints = make_int_queue();

        assert_eq!(ints.dequeue(), Some(0));
        assert_eq!(ints.dequeue(), Some(1));
        assert_eq!(ints.dequeue(), Some(2));
        assert_eq!(ints.dequeue(), None);
    }

    #[test]
    fn peeking_works() {
        let ints = make_int_queue();
        let empty_queue = Queue::<i32>::new();

        assert_eq!(ints.peek(), Some(&0));
        assert_eq!(empty_queue.peek(), None);
    }

    #[test]
    fn into_iter_works() {
        let ints = make_int_queue();
        let mut iter = ints.into_iter();

        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), None);
    }
}
