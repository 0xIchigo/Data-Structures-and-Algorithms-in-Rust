// Insertion sort is a simple sorting algorithm that builds the final sorted list one element at a time. It
// is much less efficient than more advanced algoriths, such as quick sort or merge sort, on larger lists.
// Insertion sort does, however, bolsters several advantages given its simple implementation, effectiveness 
// on small data sets, its adaptive nature, and it is more efficient in practice than other simple quadratic
// algorithms such as bubble or selection sort.

// In terms of runtime complexity, an insertion sort makes at most O(n^2) comparisons and O(n^2) swaps where
// n is the length of the list

fn insertion_sort<T: PartialOrd + std::clone::Clone>(array: &mut [T]) -> Vec<T> {
    if array.len() == 0 {
        return vec![];
    }

    let length = array.len();

    for i in 1..length {
        let mut j = i - 1;
        let mut k = array[i].clone();

        while array[j] > k {
            array[j + 1] = array[j].clone();
            if j == 0 {
                break;
            }

            j -= 1;
        }

        if j == 0 && array[0] > k {
            array[0] = k;
        } else {
            array[j + 1] = k;
        }
    }

    array.to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insertion_sort() {

        let mut test1 = vec![12, 39, 4, 36, 777];
        assert_eq!(insertion_sort(&mut test1), vec![4, 12, 36, 39, 777]);

        let mut test2 = vec![21, 55, 14, -123, 32, 0];
        assert_eq!(insertion_sort(&mut test2), vec![-123, 0, 14, 21, 32, 55]);

        let mut test3 = vec!["Orange", "Pear", "Apple", "Grape", "Banana"];
        assert_eq!(insertion_sort(&mut test3), vec!["Apple", "Banana", "Grape", "Orange", "Pear"]);
    }
}