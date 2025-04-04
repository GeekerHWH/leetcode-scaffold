pub struct MaxHeap<'a> {
    array: &'a mut Vec<i32>,
    heap_size: i32,
}

impl<'a> MaxHeap<'a> {
    pub fn new(array: &'a mut Vec<i32>) -> Self {
        let heap_size = array.len() as i32;
        MaxHeap {
            array: array,
            heap_size: heap_size,
        }
    }

    pub fn get(&self, heap_index: usize) -> i32 {
        return self.array[heap_index];
    }

    pub fn parent(&self, i: usize) -> usize {
        return (i - 1) / 2;
    }

    pub fn left(&self, i: usize) -> usize {
        return 2 * i + 1;
    }

    pub fn right(&self, i: usize) -> usize {
        return 2 * i + 2;
    }

    pub fn max_heapify(&mut self, i: usize) {
        let l: usize = self.left(i);
        let r: usize = self.right(i);

        // find largest element's index
        let mut largest: usize;
        if (l as i32) < self.heap_size && self.array[i] < self.array[l] {
            largest = l
        } else {
            largest = i
        }
        if (r as i32) < self.heap_size && self.array[largest] < self.array[r] {
            largest = r
        }

        if largest != i {
            (self.array[largest], self.array[i]) = (self.array[i], self.array[largest]);
            self.max_heapify(largest);
        }
    }

    pub fn build_heap(&mut self, n: i32) {
        self.heap_size = n;
        let mut i: i32 = (self.heap_size - 1) / 2;
        while i >= 0 {
            self.max_heapify(i as usize);
            i -= 1;
        }
    }
}

/// a rust port of heap_sort using MaxHeap
/// ## Example
/// ```
/// use leetcode_scaffold::sorts::heapsort::heap_sort;
///
/// fn test_heap_sort() {
///     let mut array = vec![5, 4, 2, 3, 1, 0];
///     heap_sort(&mut array);
///     assert_eq!(0, array[0]);
///     assert_eq!(1, array[1]);
///     assert_eq!(2, array[2]);
///     assert_eq!(5, array[5]);
/// }
/// ```
pub fn heap_sort(array: &mut Vec<i32>) {
    let mut max_heap = MaxHeap::new(array);
    let mut i = (max_heap.heap_size - 1) as usize;
    while i > 0 {
        (max_heap.array[0], max_heap.array[i]) = (max_heap.array[i], max_heap.array[0]);
        max_heap.heap_size -= 1;
        max_heap.build_heap(i as i32);
        i -= 1;
    }
}

pub struct MaxPriorityQueue<'a> {
    max_heap: &'a mut MaxHeap<'a>,
}

impl<'a> MaxPriorityQueue<'a> {
    pub fn new(max_heap: &'a mut MaxHeap<'a>) -> Self {
        max_heap.build_heap(max_heap.array.len() as i32);
        return MaxPriorityQueue { max_heap };
    }

    pub fn maximum(&self) -> i32 {
        return self.max_heap.get(0);
    }

    pub fn extract(&mut self) -> i32 {
        (
            self.max_heap.array[0],
            self.max_heap.array[(self.max_heap.heap_size - 1) as usize],
        ) = (
            self.max_heap.array[(self.max_heap.heap_size - 1) as usize],
            self.max_heap.array[0],
        );

        self.max_heap.heap_size -= 1;
        self.max_heap.max_heapify(0);
        self.max_heap.get(self.max_heap.heap_size as usize)
    }

    pub fn update_priority(&mut self, mut index: usize, value: i32) -> usize {
        if self.max_heap.array[index] > value {
            return index;
        }
        self.max_heap.array[index] = value;
        while self.max_heap.array[index] > self.max_heap.array[self.max_heap.parent(index)] {
            // swap
            let parent_index = self.max_heap.parent(index);
            (
                self.max_heap.array[index],
                self.max_heap.array[parent_index],
            ) = (
                self.max_heap.array[parent_index],
                self.max_heap.array[index],
            );
            index = parent_index;

            // prevent usize smaller than zero, which will cause panic
            if index == 0 {
                break;
            }
        }
        return index;
    }

    pub fn insert(&mut self, value: i32) -> usize {
        let n = self.max_heap.heap_size as usize;
        if n < self.max_heap.array.len() {
            self.max_heap.array[n] = value;
            self.max_heap.heap_size += 1;
            let relocated_index = self.update_priority(n, value);
            return relocated_index;
        } else {
            // n is only possible to be equal to array.len
            self.max_heap.array.push(value);
            self.max_heap.heap_size += 1;
            let relocated_index = self.update_priority(n, value);
            return relocated_index;
        }
    }

    pub fn len(&self) -> i32 {
        return self.max_heap.heap_size;
    }
}
