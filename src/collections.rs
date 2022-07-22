mod queue;
pub use queue::Queue;

pub trait Collection {
    type Item;
}

pub trait RefIterable<'a> {
    type Item: 'a;
    type Iter: Iterator<Item = &'a Self::Item>;

    fn iter(self) -> Self::Iter;
}

pub trait RefMutIterable<'a> {
    type Item: 'a;
    type IterMut: Iterator<Item = &'a mut Self::Item>;

    fn iter_mut(self) -> Self::IterMut;
}

pub trait IntoIterable {
    type Item;
    type IntoIter: IntoIterator<Item = Self::Item>;

    fn into_iter(self) -> Self::IntoIter;
}
