// Quick sort is a divide-and-conquer sorting algorithm that works by selecting a pivot, or a key, element from
// the array and patrtitions the other elements into two sub-arrays depending on whether they are less than or 
// greater than the key element. These sub-arrays are then sorted recursively

// In terms of runtime complexity, a quick sort makes at most O(n^2) comparisons where n is the number of elements
// in the array

pub fn quick_sort<T: Ord + std::clone::Clone>(array: &mut [T]) -> Vec<T> {
    if array.is_empty() {
        return vec![];
    }

    let length = array.len();
    q_s(array, 0, (length - 1) as isize);
    array.to_vec()
}

pub fn q_s<T: Ord>(array: &mut [T], low: isize, high: isize) {
    if low < high {
        let p = partrition(array, low, high);
        
        q_s(array, low, p - 1);
        q_s(array, p + 1, high);
    }
}

pub fn partrition<T: Ord>(array: &mut [T], low: isize, high: isize) -> isize {
    let key = high as usize;
    let mut index = low - 1;
    let mut last_index = high;

    loop {
        index += 1;

        while array[index as usize] < array[key] {
            index += 1;
        }

        last_index -= 1;

        while last_index >= 0 && array[last_index as usize] > array[key] {
            last_index -= 1;
        }

        if index >= last_index {
            break;
        } else {
            array.swap(index as usize, last_index as usize);
        }
    }

    array.swap(index as usize, key as usize);
    index
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quick_sort() {

        let mut test1 = [12, 39, 4, 36, 777];
        assert_eq!(quick_sort(&mut test1), [4, 12, 36, 39, 777]);

        let mut test2 = [21, 55, 14, -123, 32, 0];
        assert_eq!(quick_sort(&mut test2), [-123, 0, 14, 21, 32, 55]);

        let mut test3 = ["Orange", "Pear", "Apple", "Grape", "Banana"];
        assert_eq!(quick_sort(&mut test3), ["Apple", "Banana", "Grape", "Orange", "Pear"]);
    }
}