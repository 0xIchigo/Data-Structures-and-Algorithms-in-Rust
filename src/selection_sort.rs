// Selection sort is an in-place comparison sorting algorithm that sorts an array by
// repeatedly finding the minimum element from the unsorted part of the array and putting
// it at the beginning. Thus, in every iteration of the selection sort, the minimum
// element from the unsorted subarray is picked and moved to the sorted subarray

// In terms of runtime complexity, a selection sort makes at most O(n^2) comparisons
// making it rather inefficient on larger lists and generally performs worse than 
// insertion sort, a similar sorting algorithm. It is, however, noted for its simplicity and
// has performance advantages over more complicated algorithms in certain situations,
// particularly where auxiliary memory is limited

fn selection_sort<T: PartialOrd + std::clone::Clone>(array: &mut [T]) -> Vec<T> {
    if array.len() == 0 {
        return vec![];
    }

    let (minimum_index, _) = array.iter().enumerate().min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap()).unwrap();
    array.swap(0, minimum_index);

    if array.len() > 1 {
        selection_sort(&mut array[1..]);
    }

    array.to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_selection_sort() {

        let mut test1 = vec![12, 39, 4, 36, 777];
        assert_eq!(selection_sort(&mut test1), vec![4, 12, 36, 39, 777]);

        let mut test2 = vec![21, 55, 14, -123, 32, 0];
        assert_eq!(selection_sort(&mut test2), vec![-123, 0, 14, 21, 32, 55]);

        let mut test3 = vec!["Orange", "Pear", "Apple", "Grape", "Banana"];
        assert_eq!(selection_sort(&mut test3), vec!["Apple", "Banana", "Grape", "Orange", "Pear"]);
    }
}
