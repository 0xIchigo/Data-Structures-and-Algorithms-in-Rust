// Shell sort, also known as Shell's method, is an in-place comparison sorting algorithm 
// seen as a more generalized version of the insertion sort algorithm. The algorithm works 
// by sorting pair elements far apart from one another, and reduces the gap between elements 
// to be compared

// In terms of time complexity, a shell sort makes at most O(n^2) where n is the number
// of comparisons. This, however, is subject to contenscious debate as the runtime of a shell sort
// algorithm depends on the gap sequenced; the best known gap sequence has a worst-case
// performance of O(nlog n) comparisons

pub fn shell_sort<T: Ord + std::clone::Clone>(array: &mut [T]) -> Vec<T> { 
    if array.is_empty() {
        return vec![];
    }

    let length = array.len();
    let mut gap = length / 2;

    while gap > 0 {
        for i in gap..length {
            let temp = array[i].clone();
            let mut j = i.clone();
            while j >= gap && array[j - gap] > temp {
                array[j] = array[j - gap].clone();
                j -= gap;
            }
            array[j] = temp;
        }
        gap /= 2;
    }

    array.to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shell_sort() {

        let mut test1 = vec![12, 39, 4, 36, 777];
        assert_eq!(shell_sort(&mut test1), vec![4, 12, 36, 39, 777]);

        let mut test2 = vec![21, 55, 14, -123, 32, 0];
        assert_eq!(shell_sort(&mut test2), vec![-123, 0, 14, 21, 32, 55]);

        let mut test3 = vec!["Orange", "Pear", "Apple", "Grape", "Banana"];
        assert_eq!(shell_sort(&mut test3), vec!["Apple", "Banana", "Grape", "Orange", "Pear"]);
    }
}