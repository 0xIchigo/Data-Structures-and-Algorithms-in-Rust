// A binary search (half-interval search/logarithmic search/binary chop) is a search algorithm that
// finds a target value within a sorted array by comparing the target value to the middle of the array.
// If they are not equal, the remaining half in which the target value cannot lie is eliminated, and the
// search continues for each remaining half until the target value is found

// In terms of runtime complexity, binary search runs in logarithmic time making O(log n) comparisons
// where n is the number of elements in the array

pub fn binary_search<T: PartialEq + PartialOrd>(item: &T, array: &[T]) -> i32 {
    let mut index = -1;

    if array.is_empty() {
        return index;
    }

    let mut left = 0;
    let mut right = array.len() - 1;

    while left < right {
        let mid = left + (right - left) / 2;

        if &array[mid] > item {
            right = mid - 1;
        } else if &array[mid] < item {
            left = mid + 1;
        } else {
            left = mid;
            break;
        }
    }

    if &array[left] == item {
        index = left as i32;
        return index;
    } else {
        return index;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_search() {
        assert_eq!(binary_search(&12, &vec![0, 3, 4, 12, 777]), 3);
        assert_eq!(binary_search(&100, &vec![0, 3, 4, 12, 777]), -1);
        assert_eq!(binary_search(&"Apple", &vec!["Apple", "Banana", "Grape", "Orange", "Pineapple"]), 0);
    }
}