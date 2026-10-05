//! Turbofish and type-driven generics, using rand as the example.
//!
//! `random()` decides what to produce from the type it is asked for, which is
//! inference running in the opposite direction from what a JVM language does.

/// The type annotation on the binding is what selects the implementation.
pub fn coin_flip() -> bool {
    rand::random()
}

/// `::<T>` -- the turbofish -- says it at the call site instead.
pub fn unit_interval() -> f64 {
    rand::random::<f64>()
}

/// Shuffling in place needs a mutable borrow of the slice.
pub fn roll_dice(count: usize) -> Vec<u8> {
    (0..count).map(|_| (rand::random::<u8>() % 6) + 1).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unit_interval_is_in_range() {
        for _ in 0..1000 {
            let value = unit_interval();
            assert!((0.0..1.0).contains(&value), "out of range: {value}");
        }
    }

    #[test]
    fn dice_are_in_range_and_the_right_count() {
        let rolls = roll_dice(100);
        assert_eq!(100, rolls.len());
        assert!(rolls.iter().all(|roll| (1..=6).contains(roll)));
    }

    #[test]
    fn coin_flip_eventually_yields_both_faces() {
        let flips: Vec<bool> = (0..200).map(|_| coin_flip()).collect();
        assert!(flips.iter().any(|flip| *flip));
        assert!(flips.iter().any(|flip| !*flip));
    }
}
