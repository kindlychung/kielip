use std::sync::Arc;

use druid::Data;
use druid::{im::Vector, widget::ListIter};

#[derive(Debug, Data, Clone, PartialEq)]
pub struct RingBuffer<T: Data + Clone + Default> {
    data: Vector<T>,
    capacity: usize,
    last: usize,
}

impl<T: Data + Clone + Default> RingBuffer<T> {
    pub fn clear(self: &mut RingBuffer<T>) {
        self.data.clear();
        self.last = 0;
    }
    pub fn new(capacity: usize) -> RingBuffer<T> {
        RingBuffer {
            data: Vector::new(),
            capacity,
            last: 0,
        }
    }
    pub fn push(self: &mut RingBuffer<T>, item: T) {
        if self.data.len() < self.capacity {
            self.data.push_back(item);
            return;
        }
        self.data.set(self.last, item);
        if self.last + 1 >= self.capacity {
            self.last = 0;
        } else {
            self.last += 1;
        }
    }
    pub fn to_vec(self: &RingBuffer<T>) -> Vec<T> {
        let mut v = Vec::with_capacity(self.capacity);
        self.for_each(|item, i| v.push(item.clone()));
        v
    }
    pub fn set_capacity(self: &mut RingBuffer<T>, capacity: usize) {
        self.capacity = capacity;
        let v = self.to_vec();
        for (i, item) in v.iter().enumerate() {
            self.data.set(i, item.clone());
        }
        self.last = 0;
    }
}

impl<T: Data + Clone + Default> ListIter<T> for RingBuffer<T> {
    fn for_each(&self, mut cb: impl FnMut(&T, usize)) {
        for i in self.last..self.capacity {
            if let Some(item) = self.data.get(i) {
                cb(item, i)
            }
        }
        for i in 0..self.last {
            if let Some(item) = self.data.get(i) {
                cb(item, i)
            }
        }
    }

    fn for_each_mut(&mut self, mut cb: impl FnMut(&mut T, usize)) {
        for i in self.last..self.capacity {
            if let Some(item) = self.data.get_mut(i) {
                cb(item, i)
            }
        }
        for i in 0..self.last {
            if let Some(item) = self.data.get_mut(i) {
                cb(item, i)
            }
        }
    }

    fn data_len(&self) -> usize {
        self.data.data_len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buffer() {
        let mut buffer = RingBuffer::new(2);
        buffer.push(1);
        assert_eq!(buffer.to_vec(), vec![1]);
        buffer.push(2);
        assert_eq!(buffer.to_vec(), vec![1, 2]);
        buffer.push(3);
        assert_eq!(buffer.to_vec(), vec![2, 3]);
        buffer.push(4);
        assert_eq!(buffer.to_vec(), vec![3, 4]);
        buffer.push(5);
        assert_eq!(buffer.to_vec(), vec![4, 5]);
        buffer.set_capacity(5);
        buffer.push(6);
        assert_eq!(buffer.to_vec(), vec![4, 5, 6]);
        buffer.push(7);
        assert_eq!(buffer.to_vec(), vec![4, 5, 6, 7]);
        buffer.push(8);
        assert_eq!(buffer.to_vec(), vec![4, 5, 6, 7, 8]);
        buffer.push(9);
        assert_eq!(buffer.to_vec(), vec![5, 6, 7, 8, 9]);
    }
}
