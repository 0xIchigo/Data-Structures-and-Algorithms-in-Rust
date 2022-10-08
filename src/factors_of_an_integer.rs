// This algorithm computes the factors of a positive integer. These factors are positive
// integers by which the number being factored can be divided to yield a positive integer
// result. Prime numbers will only return a vector of two factors: 1 and itself

fn main() {
    assert_eq!(vec![1, 2, 7, 14], factor(14));
    assert_eq!(vec![1, 3], factor(3));
    assert_eq!(vec![1, 2, 4, 5, 10, 10, 20, 25, 50, 100], factor(100));
}

fn factor(num: i32) -> Vec<i32> {
    let mut factors: Vec<i32> = Vec::new();

    for i in 1..((num as f32).sqrt() as i32 + 1) {
        if num % i == 0 {
            factors.push(i);
            factors.push(num / i);
        }
    }

    factors.sort();
    factors
}

fn alternative_factor_function(n: i32) -> Vec<i32> {
    (1..=n).filter(|i| n % i == 0).collect()
}