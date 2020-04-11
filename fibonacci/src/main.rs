fn main() {
    let n = 20;
    println!("{}", fibonacci(n));
    println!("{}", fibonacci_iterative(n));
}

fn fibonacci(n: usize) -> usize {
    if n == 1 || n == 2 {
        1
    } else {
        fibonacci(n - 2) + fibonacci(n - 1)
    }
}

fn fibonacci_iterative(n: usize) -> usize {
    let mut current_num = 1;
    let mut previous_num = 1;
    for _ in 2..n {
        let temp = current_num;
        current_num = current_num + previous_num;
        previous_num = temp;
    }
    current_num
}

#[cfg(test)]
mod fibonacci_tests {
    use super::*;

    #[test]
    fn test_fibonacci() {
        assert_eq!(fibonacci(1), 1);
        assert_eq!(fibonacci(2), 1);
        assert_eq!(fibonacci(3), 2);
        assert_eq!(fibonacci(4), 3);
        assert_eq!(fibonacci(5), 5);
        assert_eq!(fibonacci(6), 8);
    }

    #[test]
    fn test_fibonacci_iterative() {
        assert_eq!(fibonacci_iterative(1), 1);
        assert_eq!(fibonacci_iterative(2), 1);
        assert_eq!(fibonacci_iterative(3), 2);
        assert_eq!(fibonacci_iterative(4), 3);
        assert_eq!(fibonacci_iterative(5), 5);
        assert_eq!(fibonacci_iterative(6), 8);
    }

    #[test]
    #[should_panic(expected = "attempt to add with overflow")]
    fn test_fibonacci_60() {
        fibonacci_iterative(100);
    }
}
