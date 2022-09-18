// Pascal's triangle is a triangular array of the binomial coefficients that arises
// in probability theory, combinatorics, and algebra named after the French mathematician
// Blaise Pascal. The triangle begins with a one at the top and with ones running down
// the two sides of the traingle. Each new number lies between two numbers and below them,
// and its value is the sum of the two numbers above it

// In terms of runtime complexity, a Pascal's triangle makes at most O(n^2) calculations

fn pascals_triangle(rows: i32) -> Vec<Vec<i32>> {
    let mut triangle: Vec<Vec<i32>> = vec![];

    for i in 1..rows + 1 {
        let mut vec: Vec<i32> = vec![1];
        let mut j: i32 = 1;

        for k in 1..i {
            j *= i - k;
            j /= k;

            vec.push(j);
        }
        triangle.push(vec);
    }

    triangle
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pascal_triangle() {
        assert_eq!(pascals_triangle(1), vec![vec![1]]);
        assert_eq!(pascals_triangle(2), vec![vec![1], vec![1, 1]]);
        assert_eq!(pascals_triangle(3), vec![vec![1], vec![1, 1], vec![1, 2, 1]]);
        assert_eq!(pascals_triangle(4), vec![vec![1], vec![1, 1], vec![1, 2, 1], vec![1, 3, 3, 1]]);
    }
}