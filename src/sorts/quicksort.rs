// normal partition that choose the last element as pivot
// for better time cost, we should implement another randomized_partition
fn partition<T>(array: &mut Vec<T>, l: usize, r: usize) -> usize
where
    T: PartialEq + PartialOrd + Copy,
{
    let pivot = array[r];
    let mut low_side_upper_bound = l as i32 - 1;

    for j in l..=r - 1 {
        if array[j] <= pivot {
            low_side_upper_bound += 1;
            (array[low_side_upper_bound as usize], array[j]) =
                (array[j], array[low_side_upper_bound as usize])
        }
    }

    (array[(low_side_upper_bound + 1) as usize], array[r]) =
        (array[r], array[(low_side_upper_bound + 1) as usize]);
    return (low_side_upper_bound + 1) as usize;
}

fn randomized_partition<T>(array: &mut Vec<T>, l: usize, r: usize) -> usize
where
    T: PartialEq + PartialOrd + Copy,
{
    // 1. choose pivot_index randomly.
    let pivot_index = rand::random_range(l..r);
    // 2. exchange pivot with array[r] to reuse partition
    (array[pivot_index], array[r]) = (array[r], array[pivot_index]);
    // 3. use normal partition
    partition(array, l, r)
}

pub fn quick_sort<T>(array: &mut Vec<T>, l: usize, r: usize)
where
    T: PartialEq + PartialOrd + Copy,
{
    if l < r {
        let pivot_index = partition(array, l, r);
        if pivot_index > 0 {
            quick_sort(array, l, pivot_index - 1);
        }
        quick_sort(array, pivot_index + 1, r);
    }
}

pub fn randomized_quick_sort<T>(array: &mut Vec<T>, l: usize, r: usize)
where
    T: PartialEq + PartialOrd + Copy,
{
    if l < r {
        let pivot_index = randomized_partition(array, l, r);
        if pivot_index > 0 {
            randomized_quick_sort(array, l, pivot_index - 1);
        }
        randomized_quick_sort(array, pivot_index + 1, r);
    }
}

#[cfg(test)]
mod test_quick_sort {

    use super::partition;
    use super::quick_sort;
    use super::randomized_quick_sort;

    #[test]
    fn test_partition() {
        let mut array = vec![0, 2, 6, 3, 4, 5, 1];
        let pivot_index = partition(&mut array, 0, 6);
        assert_eq!(1, pivot_index);
        assert_eq!(vec![0, 1, 6, 3, 4, 5, 2], array)
    }

    #[test]
    fn test_quick_sort() {
        let mut array = vec![0, 0, 0, 0, 0, 0, 0];
        quick_sort(&mut array, 0, 6);
        assert_eq!(vec![0, 0, 0, 0, 0, 0, 0], array);

        let mut array_2 = vec![6, 5, 4, 3, 2, 1, 0];
        quick_sort(&mut array_2, 0, 6);
        assert_eq!(vec![0, 1, 2, 3, 4, 5, 6], array_2);

        let mut array_3: Vec<i32> = vec![];
        quick_sort(&mut array_3, 0, 0);
        assert_eq!(array_3, array_3);

        let mut array_4 = vec![1.6, 1.5, 1.4, 1.3, 1.2, 1.1, 0.0];
        quick_sort(&mut array_4, 0, 6);
        assert_eq!(vec![0.0, 1.1, 1.2, 1.3, 1.4, 1.5, 1.6], array_4);
    }

    #[test]
    fn test_randomized_quick_sort() {
        let mut array = vec![0, 0, 0, 0, 0, 0, 0];
        randomized_quick_sort(&mut array, 0, 6);
        assert_eq!(vec![0, 0, 0, 0, 0, 0, 0], array);

        let mut array_2 = vec![6, 5, 4, 3, 2, 1, 0];
        randomized_quick_sort(&mut array_2, 0, 6);
        assert_eq!(vec![0, 1, 2, 3, 4, 5, 6], array_2);

        let mut array_3: Vec<i32> = vec![];
        randomized_quick_sort(&mut array_3, 0, 0);
        assert_eq!(array_3, array_3);

        let mut array_4 = vec![1.6, 1.5, 1.4, 1.3, 1.2, 1.1, 0.0];
        randomized_quick_sort(&mut array_4, 0, 6);
        assert_eq!(vec![0.0, 1.1, 1.2, 1.3, 1.4, 1.5, 1.6], array_4);
    }
}
