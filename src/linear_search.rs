// A linear search is a method for finding an element within a list by checking each element of
// the list sequentially until the desired element is found, or the entire list has been searched through.

// In terms of runtime complexity, a linear search makes at most n comparisons where n is the length of the list

pub fn linear_search<T: PartialEq>(item: &T, array: &[T]) -> i32{
    let mut index_position = -1;

    for (index, data) in array.iter().enumerate() {
        if item == data {
            index_position = index as i32;
            return index_position;
        }
    }

    index_position
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear_search() {
        assert_eq!(linear_search(&12, &vec![12, 3, 4, 36, 777]), 0);
        assert_eq!(linear_search(&100, &vec![12, 3, 4, 36, 777]), -1);
        assert_eq!(linear_search(&"Apple", &vec!["Orange", "Pear", "Apple", "Grape", "Banana"]), 2);
    }
}