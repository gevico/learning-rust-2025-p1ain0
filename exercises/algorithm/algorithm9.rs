/*
	heap
	This question requires you to implement a binary heap function
*/

use std::cmp::Ord;
use std::default::Default;

pub struct Heap<T>
where
    T: Default,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()],
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// 插入堆（上浮）
    pub fn add(&mut self, value: T) {
        self.items.push(value);
        self.count += 1;

        let mut idx = self.count;
        while idx > 1 {
            let p = self.parent_idx(idx);
            if !(self.comparator)(&self.items[idx], &self.items[p]) {
                break;
            }
            self.items.swap(idx, p);
            idx = p;
        }
    }
    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    fn smallest_child_idx(&self, idx: usize) -> usize {
        let left = self.left_child_idx(idx);
        let right = self.right_child_idx(idx);

        if right > self.count {
            return left;
        }

        if (self.comparator)(&self.items[left], &self.items[right]) {
            left
        } else {
            right
        }
    }
    fn sift_down(&mut self, mut idx: usize) {
        while self.children_present(idx) {
            let smallest_child_idx = self.smallest_child_idx(idx);
            
            // If the current node already satisfies the heap property with its children, stop.
            if !(self.comparator)(&self.items[smallest_child_idx], &self.items[idx]) {
                break;
            }

            // Otherwise, swap with the child that has higher priority.
            self.items.swap(idx, smallest_child_idx);
            idx = smallest_child_idx; // Move down to the child's position
        }
    }
}

impl<T> Heap<T>
where
    T: Default + Ord,
{
    /// Create a new MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        // The root element (highest priority) is at index 1.
        // We need to remove it and replace it with the last element.
        // Then, we sift down the new root to maintain the heap property.
        
        // 1. Take the root element out by swapping it with the default value.
        //    We temporarily place the default value at index 1.
        let root_index = 1;
        let last_index = self.count; // Current last valid index
        
        // If the heap has only one element, we can just pop it directly.
        if last_index == 1 {
            self.count = 0;
            return Some(self.items.pop().unwrap()); // This removes and returns the root/default
        }

        // 2. Swap the root with the last element.
        self.items.swap(root_index, last_index);
        
        // 3. Pop the old root (now at the end) and decrement the count.
        let popped_root = self.items.pop().unwrap();
        self.count -= 1;

        // 4. Sift down the new root (which was previously at the end).
        //    We only need to sift down if there are still elements left after popping.
        if self.count > 0 {
            self.sift_down(root_index);
        }

        // 5. Return the original root element that was popped.
        Some(popped_root)
    }

}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}