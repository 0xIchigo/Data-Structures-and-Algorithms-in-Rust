// Gnome sort is a variation of the insertion sort algorithm that does not use nested loops. It was initially 
// proposed by Hamid Sarbazi-Azad in 2000 and was refered to as stupid sort, Dutch computer scientist Dick 
// Grune renamed it the gnome sort in light of a story describing how Dutch Garden Gnomes sort their flower pots

// In terms of runtime complexity, a gnome sort makes at most O(n^2) comparisons and O(n^2) swaps where n is the 
// length of the list


fn gnome_sort<T: PartialOrd + std::clone::Clone>(array: &mut [T]) -> Vec<T> {
    if array.is_empty() {
        return vec![];
    }

    let length = array.len();
    let mut i: usize = 1;
    let mut j: usize = 2;

    while i < length {
        if array[i - 1] < array[i] {
            i = j;
            j = i + 1;
        } else {
            array.swap(i - 1, i);
            i -= 1;

            if i == 0 {
                i = j;
                j += 1;
            }
        }
    }

    array.to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gnome_sort() {

        let mut test1 = vec![12, 39, 4, 36, 777];
        assert_eq!(gnome_sort(&mut test1), vec![4, 12, 36, 39, 777]);

        let mut test2 = vec![21, 55, 14, -123, 32, 0];
        assert_eq!(gnome_sort(&mut test2), vec![-123, 0, 14, 21, 32, 55]);

        let mut test3 = vec!["Orange", "Pear", "Apple", "Grape", "Banana"];
        assert_eq!(gnome_sort(&mut test3), vec!["Apple", "Banana", "Grape", "Orange", "Pear"]);
    }
}