//! Iterators and closures. Iterators are lazy, so a chain does no work until
//! something consumes it.

/// A lazy chain: nothing runs until `collect` asks for values.
pub fn even_squares(limit: u32) -> Vec<u32> {
    (1..=limit)
        .filter(|candidate| candidate % 2 == 0)
        .map(|candidate| candidate * candidate)
        .collect()
}

/// `fold` with an explicit accumulator, the general case the other adapters are
/// special cases of.
pub fn sum_of_lengths(words: &[&str]) -> usize {
    words.iter().fold(0, |total, word| total + word.len())
}

/// Closures capture their environment. This one borrows `factor` immutably.
pub fn scale_all(values: &[i32], factor: i32) -> Vec<i32> {
    values.iter().map(|value| value * factor).collect()
}

/// Returning a closure needs `impl Fn`, because every closure has its own
/// anonymous type.
pub fn adder(amount: i32) -> impl Fn(i32) -> i32 {
    move |value| value + amount
}

/// An infinite iterator, made finite by `take`. Proof that laziness is real:
/// without it this would never terminate.
pub fn first_fibonacci(count: usize) -> Vec<u64> {
    let mut state = (0u64, 1u64);
    std::iter::from_fn(move || {
        let current = state.0;
        state = (state.1, state.0 + state.1);
        Some(current)
    })
    .take(count)
    .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adapters_compose() {
        assert_eq!(vec![4, 16, 36, 64, 100], even_squares(10));
    }

    #[test]
    fn fold_accumulates() {
        assert_eq!(12, sum_of_lengths(&["four", "five", "nine"]));
    }

    #[test]
    fn closures_capture_their_environment() {
        assert_eq!(vec![3, 6, 9], scale_all(&[1, 2, 3], 3));

        let add_ten = adder(10);
        assert_eq!(15, add_ten(5));
        assert_eq!(10, add_ten(0));
    }

    #[test]
    fn infinite_iterators_are_safe_when_bounded() {
        assert_eq!(vec![0, 1, 1, 2, 3, 5, 8, 13], first_fibonacci(8));
    }
}
