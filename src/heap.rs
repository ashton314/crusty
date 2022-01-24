// Generic heap

#[derive(Debug)]
pub struct Heap<T> where
    T: PartialOrd {
    heap: Vec<T>
}

impl<T> Heap<T> where
    T: PartialOrd {

    #[allow(unused_mut)]
    pub fn new() -> Heap<T> {
        let mut the_heap = Vec::new();

        Heap {
            heap: the_heap
        }
    }

    pub fn push(&mut self, val: T) {
        self.heap.push(val);
        self.bubble_up();
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.heap.len() == 0 {
            None
        }
        else {
            let ret = self.heap.swap_remove(0);
            self.sift_down(0);
            Some(ret)
        }
    }

    fn sift_down(&mut self, sift: usize) {
        let (chld_lix, chld_rix) = child_idxs(sift);
        match (self.heap.get(chld_lix), self.heap.get(chld_rix)) {
            (Some(a), Some(b)) => {
                let (idx, smallest) = if a <= b { (chld_lix, a) } else { (chld_rix, b) };
                if self.heap[sift] > *smallest {
                    self.heap.swap(sift, idx);
                    self.sift_down(idx)
                }
            },
            (Some(a), None) => {
                if self.heap[sift] > *a {
                    self.heap.swap(chld_lix, sift);
                    self.sift_down(chld_lix)
                }
            },
            (None, None) => (),
            (None, _) => panic!("Malformed heap! Missing left element!")
        }
    }

    fn bubble_up(&mut self) {
        self.bubble_up_n(self.heap.len() - 1);
    }

    fn bubble_up_n(&mut self, bubble: usize) {
        if let Some(parent) = parent_idx(bubble) {
            if self.heap[bubble] < self.heap[parent] {
                self.heap.swap(bubble, parent);

                self.bubble_up_n(parent);
            }
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
    ((2 * a) + 1, (2 * a) + 2)
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

    #[test]
    fn child_assumptions() {
        assert_eq!(child_idxs(0), (1, 2));
        assert_eq!(child_idxs(1), (3, 4));
        assert_eq!(child_idxs(2), (5, 6));
        assert_eq!(child_idxs(3), (7, 8));
        assert_eq!(child_idxs(4), (9, 10));
    }

    #[test]
    fn new_and_insert() {
        let mut foo = Heap::new();
        foo.push(10);
        foo.push(42);
        foo.push(5);
        foo.push(3);
        foo.push(4);
        assert_eq!(foo.heap, vec![3, 4, 10, 42, 5])
    }

    #[test]
    fn pop_off() {
        let mut foo = Heap::new();
        foo.push(10);
        foo.push(42);
        foo.push(5);
        foo.push(3);
        foo.push(4);
        assert_eq!(foo.heap, vec![3, 4, 10, 42, 5]);

        assert_eq!(foo.pop(), Some(3));
        assert_eq!(foo.pop(), Some(4));
        assert_eq!(foo.pop(), Some(5));
        assert_eq!(foo.pop(), Some(10));
        assert_eq!(foo.pop(), Some(42));
        assert_eq!(foo.pop(), None);
        assert_eq!(foo.pop(), None);
    }
}
