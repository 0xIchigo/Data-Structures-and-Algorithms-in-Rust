// Circle sort is a sorting algorithm in which concentric circles are made on an array and the elements
// lying on the same circle diametrically opposite to each other are compared, and if the element on the
// left hand side is greater than that on the right hand side then the elements are swapped. This process
// is repeated in a recursive manner, dividing the array into sub-arrays, until the array is sorted

// Circle sort is an unstable, recursive, parallelizable, in place sorting algorithm. It is one of the
// fastest ways to sort an inverted array with an average time complexity of O(n log(n)) and space 
// complexity of O(1). Description from: https://iq.opengenus.org/circle-sort/

fn _circle_sort<T: PartialOrd>(array: &mut[T], low: usize, high: usize, swaps: usize) -> usize {
    if low == high {
        return swaps;
    }

    let mut lo = low;
    let mut hi = high;
    let mid = (hi - low) / 2;
    let mut s = swaps;

    while lo < hi {
        if array[lo] > array[hi] {
            array.swap(lo, hi);
            s += 1;
        }

        lo += 1;
        hi -= 1;
    }

    if lo == hi {
        if array[lo] > array[hi + 1] {
            array.swap(lo, hi + 1);
            s += 1;
        }
    }

    s = _circle_sort(array, low, low + mid, s);
    s = _circle_sort(array, low + mid + 1, high, s);

    return s;
}

fn circle_sort<T: PartialOrd + std::clone::Clone>(array: &mut[T]) -> Vec<T> {
    let len = array.len();

    if array.is_empty() {
        return vec![];
    }

    loop {
        if _circle_sort(array, 0, len - 1, 0) == 0 {
            break;
        }
    }

    array.to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circle_sort() {

        let mut test1 = vec![12, 39, 4, 36, 777];
        assert_eq!(circle_sort(&mut test1), vec![4, 12, 36, 39, 777]);

        let mut test2 = vec![21, 55, 14, 123, 32, 0];
        assert_eq!(circle_sort(&mut test2), vec![0, 14, 21, 32, 55, 123]);

    }
}