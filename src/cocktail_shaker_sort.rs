// The cocktail shaker sort, also known as bidirectional bubble sort, is an extension of
// bubble sort such that it operates in two directions. While it improves on bubble sort by
// moving items to the beginning of the list more quickly, it provides only marginal
// performance improvements which is why it is used primarily as an education tool 

// In terms of runtime complexity, a cocktail shaker sort makes at most O(n^2) comparisons 
// and O(n^2) swaps where n is the length of the list

fn cocktail_shaker_sort<T: PartialOrd + std::clone::Clone>(array: &mut [T]) -> Vec<T> {
    if array.is_empty() {
        return vec![];
    }

    let length = array.len();

    loop {
        let mut swapped = false;
        let mut i = 0;

        while i + 1 < length {
            if array[i] > array[i + 1] {
                array.swap(i, i + 1);
                swapped = true;
            }

            i += 1;
        }

        if swapped {
            swapped = false;
            i = length - 1;

            while i > 0 {
                if array[i - 1] > array[i] {
                    array.swap(i, i - 1);
                    swapped = true;
                }

                i -= 1;
            }
        }

        if !swapped {
            break;
        }
    }

    array.to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cocktail_shaker_sort() {

        let mut test1 = vec![12, 39, 4, 36, 777];
        assert_eq!(cocktail_shaker_sort(&mut test1), vec![4, 12, 36, 39, 777]);

        let mut test2 = vec![21, 55, 14, -123, 32, 0];
        assert_eq!(cocktail_shaker_sort(&mut test2), vec![-123, 0, 14, 21, 32, 55]);

        let mut test3 = vec!["Orange", "Pear", "Apple", "Grape", "Banana"];
        assert_eq!(cocktail_shaker_sort(&mut test3), vec!["Apple", "Banana", "Grape", "Orange", "Pear"]);
    }
}