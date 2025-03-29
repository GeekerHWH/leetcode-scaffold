use leetcode_scaffold::sorts;

#[cfg(test)]
mod tests {
    use leetcode_scaffold::sorts::heapsort::heap_sort;

    use super::*;

    #[test]
    fn test_parent_left_right() {
        //             0
        //           /   \
        //          1     2
        //         / \   /
        //        3   4 5
        let mut array = vec![0, 1, 2, 3, 4, 5];
        let index_is_value_heap = sorts::heapsort::MaxHeap::new(&mut array);

        assert_eq!(1, index_is_value_heap.parent(3));
        assert_eq!(1, index_is_value_heap.parent(4));
        assert_eq!(0, index_is_value_heap.parent(1));

        assert_eq!(0, index_is_value_heap.parent(2));
        assert_eq!(2, index_is_value_heap.parent(5))
    }

    #[test]
    fn test_max_heapify() {
        let mut array = vec![0, 1, 2, 3, 4, 5];
        let mut index_is_value_heap = sorts::heapsort::MaxHeap::new(&mut array);

        index_is_value_heap.max_heapify(1);
        //             0
        //           /   \
        //          4     2
        //         / \   /
        //        3   1 5
        assert_eq!(4, index_is_value_heap.get(1));
        assert_eq!(1, index_is_value_heap.get(4));
        assert_eq!(3, index_is_value_heap.get(3));

        index_is_value_heap.max_heapify(0);
        //             4
        //           /   \
        //          0     2
        //         / \   /
        //        3   1 5
        // recalled onece
        //             4
        //           /   \
        //          3     2
        //         / \   /
        //        0   1 5
        assert_eq!(4, index_is_value_heap.get(0));
        assert_eq!(3, index_is_value_heap.get(1));
        assert_eq!(2, index_is_value_heap.get(2));
    }

    #[test]
    fn test_build_heap() {
        let mut array = vec![0, 1, 2, 3, 4, 5];
        let mut index_is_value_heap = sorts::heapsort::MaxHeap::new(&mut array);

        index_is_value_heap.build_heap(6);
        //             5
        //           /   \
        //          4     2
        //         / \   /
        //        3   1 0
        assert_eq!(5, index_is_value_heap.get(0));
        assert_eq!(4, index_is_value_heap.get(1));
        assert_eq!(2, index_is_value_heap.get(2));
        assert_eq!(3, index_is_value_heap.get(3));
        assert_eq!(1, index_is_value_heap.get(4));
        assert_eq!(0, index_is_value_heap.get(5));
    }

    // It's just a draft here
    //                 0
    //            /         \
    //           1           2
    //         /   \      /     \
    //        3     4     5      6
    //       / \   / \   / \   /  \
    //      7   8 9  10 11 12 13  14

    #[test]
    fn test_heap_sort() {
        let mut array = vec![5, 4, 2, 3, 1, 0];
        heap_sort(&mut array);
        assert_eq!(0, array[0]);
        assert_eq!(1, array[1]);
        assert_eq!(2, array[2]);
        assert_eq!(5, array[5]);
    }
}
