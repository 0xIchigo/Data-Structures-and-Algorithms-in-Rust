// Counting sort is a sorting algorithm for sorting a collection of keys between a specific
// range. It operates by counting the number of objects having distinct key, and then applies
// a prefix sum on those counts to determine the positions of each key value in the output
// sequence. It is often used as a subroutine in a radix sort, another sorting algorithm which
// can handle larger keys more efficiently.

// In terms of runtime complexity, a counting sort makes at most O(n + k) calculations wherein k
// is the range of the non-negative key values

use std::ops::AddAssign;

fn counting_sort<T: Into<i32> + From<u8> + AddAssign + Copy>(array: &mut [T], max: usize) -> Vec<T> {
    let mut count_array: Vec<usize> = vec![0; max + 1];

    for &num in array.iter() {
        count_array[num.into() as usize] += 1;
    }

    let mut i: usize = 0;
    let mut data = T::from(0);

    for &number in count_array.iter() {
        for _ in 0..number {
            array[i] = data;
            i += 1;
        }

        data += T::from(1);
    }

    array.to_vec()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counting_sort() {

        let mut test1 = vec![12, 39, 4, 36, 777];
        assert_eq!(counting_sort(&mut test1, 777), vec![4, 12, 36, 39, 777]);

        let mut test2 = vec![21, 55, 14, 123, 32, 0];
        assert_eq!(counting_sort(&mut test2, 123), vec![0, 14, 21, 32, 55, 123]);

    }
}