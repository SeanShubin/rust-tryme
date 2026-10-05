//! Traits, generics and trait objects: static versus dynamic dispatch.

use std::fmt::Display;

pub trait Greeter {
    fn name(&self) -> String;

    /// A default method. Implementors may override it, but need not.
    fn greet(&self) -> String {
        format!("Hello, {}!", self.name())
    }
}

pub struct English {
    pub who: String,
}

pub struct Pirate;

impl Greeter for English {
    fn name(&self) -> String {
        self.who.clone()
    }
}

impl Greeter for Pirate {
    fn name(&self) -> String {
        "matey".to_string()
    }

    fn greet(&self) -> String {
        format!("Ahoy, {}!", self.name())
    }
}

/// Static dispatch: the compiler emits one copy per concrete type, so the call
/// is direct and inlinable.
pub fn greet_statically<G: Greeter>(greeter: &G) -> String {
    greeter.greet()
}

/// Dynamic dispatch through a trait object, which is what makes a heterogeneous
/// collection possible. The cost is a vtable lookup.
pub fn greet_all(greeters: &[Box<dyn Greeter>]) -> Vec<String> {
    greeters.iter().map(|greeter| greeter.greet()).collect()
}

/// Traits can be implemented for types you did not define, as long as the trait
/// is yours -- the orphan rule.
pub trait Describe {
    fn describe(&self) -> String;
}

impl Describe for i32 {
    fn describe(&self) -> String {
        format!("the integer {self}")
    }
}

impl<T: Display> Describe for Vec<T> {
    fn describe(&self) -> String {
        let joined = self
            .iter()
            .map(|item| item.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        format!("[{joined}]")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_method_is_used_unless_overridden() {
        let english = English { who: "world".to_string() };
        assert_eq!("Hello, world!", greet_statically(&english));
        assert_eq!("Ahoy, matey!", greet_statically(&Pirate));
    }

    #[test]
    fn trait_objects_allow_a_mixed_collection() {
        let greeters: Vec<Box<dyn Greeter>> = vec![
            Box::new(English { who: "you".to_string() }),
            Box::new(Pirate),
        ];
        assert_eq!(vec!["Hello, you!", "Ahoy, matey!"], greet_all(&greeters));
    }

    #[test]
    fn traits_extend_foreign_types() {
        assert_eq!("the integer 7", 7.describe());
        assert_eq!("[1, 2, 3]", vec![1, 2, 3].describe());
    }
}
