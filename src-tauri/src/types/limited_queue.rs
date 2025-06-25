use std::collections::VecDeque;
pub struct LimitedQueue<T> {
    size: usize,
    queue: VecDeque<T>,
}
impl<T> LimitedQueue<T> {
    pub fn new(size: usize) -> Self {
        LimitedQueue {
            size,
            queue: VecDeque::with_capacity(size),
        }
    }
    pub fn push(&mut self, item: T) {
        if self.queue.len() >= self.size {
            self.queue.pop_front();
        }
        self.queue.push_back(item);
    }
    pub fn get(&self, index: usize) -> Option<&T> {
        self.queue.get(index)
    }
}
