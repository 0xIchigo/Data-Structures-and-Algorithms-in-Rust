// The pancake sorting algorithm derives from the mathematical problem of sorting a 
// disorganized stack of pancakes in order of size when a spatula can be inserted at 
// any point in the stack and is used to flipp all pancakes above it. Pancake number 
// refers to the minimum number of flips required for a given number of pancakes.

fn pancake_sort<T: Ord + std::clone::Clone>(array: &mut [T]) -> Vec<T> {
    if array.is_empty() {
        return vec![];
    }

    let length = array.len();

    if length < 2 {
        return array.to_vec();
    }

    // Iterate in reverse order
    for i in (0..length).rev() {
        let max_index = array.iter()
            .take(i + 1)
            .enumerate()
            .max_by_key(|&(_, element) | element)
            .map(|(idx, _)| idx)
            .unwrap();
        
            if max_index != i {
                flip(array, max_index);
                flip(array, i);
            }
    }

    array.to_vec()
}

fn flip<E: PartialOrd>(array: &mut [E], num: usize) {
    array[0..num + 1].reverse();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pancake_sort() {

        let mut test1 = vec![12, 39, 4, 36, 777];
        assert_eq!(pancake_sort(&mut test1), vec![4, 12, 36, 39, 777]);

        let mut test2 = vec![21, 55, 14, -123, 32, 0];
        assert_eq!(pancake_sort(&mut test2), vec![-123, 0, 14, 21, 32, 55]);

        let mut test3 = vec!["Orange", "Pear", "Apple", "Grape", "Banana"];
        assert_eq!(pancake_sort(&mut test3), vec!["Apple", "Banana", "Grape", "Orange", "Pear"]);
    }
}