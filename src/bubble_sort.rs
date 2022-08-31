// Bubble sort is a type of sorting algorithm that repeatedly loops through elements of a list, comparing 
// the current element with the one after it, and swapping their values if necessary

// In terms of runtime complexity, a bubble sort makes at most O(n^2) comparisons and O(n^2) swaps where
// n is the length of the list

pub fn bubble_sort<T: Ord + std::clone::Clone>(array: &mut [T]) -> Vec<T> {
    if array.is_empty() {
        return vec![];
    }

    for i in 0..array.len() {
        for j in 0..array.len() - 1 - i {
            if array[j] > array[j + 1] {
                array.swap(j, j + 1);
            }
        }
    }
    
    array.to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bubble_sort() {

        let mut test1 = vec![12, 39, 4, 36, 777];
        assert_eq!(bubble_sort(&mut test1), vec![4, 12, 36, 39, 777]);

        let mut test2 = vec![21, 55, 14, -123, 32, 0];
        assert_eq!(bubble_sort(&mut test2), vec![-123, 0, 14, 21, 32, 55]);

        let mut test3 = vec!["Orange", "Pear", "Apple", "Grape", "Banana"];
        assert_eq!(bubble_sort(&mut test3), vec!["Apple", "Banana", "Grape", "Orange", "Pear"]);
    }
}