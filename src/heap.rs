// Generic heap

pub struct Heap<T> where
    T: PartialOrd {
    heap: Vec<T>
}

impl<T> Heap<T> where
    T: PartialOrd {
    pub fn push(&mut self, val: T) {
        self.heap.push(val);
        self.bubble_up();
    }

    pub fn pop(&mut self) -> Option<T> {
        None
    }

    fn bubble_up(&mut self) {
        self.bubble_up_n(self.heap.len() - 1);
    }

    fn bubble_up_n(&mut self, bubble: usize) {
        if bubble == 0 {
            return;             // at top of heap; stop now
        }

        let parent = parent_idx(bubble).unwrap();

        if self.heap[bubble] < self.heap[parent] {
            self.heap.swap(bubble, parent);

            self.bubble_up_n(parent);
        }
    }

}

fn parent_idx(a: usize) -> Option<usize> {
    if a == 0 {
        None
    }
    else {
        Some((a - 1) / 2)
    }
}

fn child_idxs(a: usize) -> (usize, usize) {
    (2 * a, (2 * a) + 1)
}

#[cfg(test)]
mod tests {
    use crate::heap::*;

    #[test]
    fn parent_assumptions() {
        assert_eq!(parent_idx(0), None);
        assert_eq!(parent_idx(1), Some(0));
        assert_eq!(parent_idx(2), Some(0));
        assert_eq!(parent_idx(3), Some(1));
        assert_eq!(parent_idx(4), Some(1));
        assert_eq!(parent_idx(5), Some(2));
        assert_eq!(parent_idx(6), Some(2));
        assert_eq!(parent_idx(7), Some(3));
    }
}
