//! Ownership, moves and borrowing -- the rules with no equivalent on the JVM.

/// Takes ownership. The caller cannot use its value afterwards.
pub fn consume(text: String) -> usize {
    text.len()
}

/// Borrows immutably. The caller keeps ownership, and may lend to many readers
/// at once.
pub fn inspect(text: &str) -> usize {
    text.len()
}

/// Borrows mutably. Only one such borrow may exist at a time, which is what
/// rules out data races at compile time.
pub fn append(text: &mut String, suffix: &str) {
    text.push_str(suffix);
}

/// Clone is explicit in Rust: copying heap data is never implicit, so the cost
/// is always visible at the call site.
pub fn duplicate(text: &str) -> String {
    text.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn borrow_leaves_the_original_usable() {
        let greeting = String::from("hello");
        assert_eq!(5, inspect(&greeting));

        // Still owned here, precisely because inspect only borrowed it.
        assert_eq!("hello", greeting);
    }

    #[test]
    fn mutable_borrow_modifies_in_place() {
        let mut greeting = String::from("hello");
        append(&mut greeting, ", world");
        assert_eq!("hello, world", greeting);
    }

    #[test]
    fn move_transfers_ownership() {
        let greeting = String::from("hello");
        assert_eq!(5, consume(greeting));

        // Uncommenting the next line fails to compile with "borrow of moved
        // value", which is the whole point of the experiment.
        // assert_eq!("hello", greeting);
    }

    #[test]
    fn clone_is_independent() {
        let original = String::from("hello");
        let mut copy = duplicate(&original);
        append(&mut copy, "!");
        assert_eq!("hello", original);
        assert_eq!("hello!", copy);
    }
}
