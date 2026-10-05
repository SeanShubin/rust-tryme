//! Enums with data, and the exhaustive pattern matching that makes them safe.

/// A sum type where each variant carries its own payload. The closest JVM
/// equivalent is a sealed interface, but here the variants are the type rather
/// than subclasses of it.
#[derive(Debug, Clone, PartialEq)]
pub enum Shape {
    Point,
    Circle { radius: f64 },
    Rectangle { width: f64, height: f64 },
    Triangle(f64, f64, f64),
}

impl Shape {
    /// The compiler rejects this function if a variant is left unhandled, so
    /// adding a variant turns every incomplete match into a build error.
    pub fn area(&self) -> f64 {
        match self {
            Shape::Point => 0.0,
            Shape::Circle { radius } => std::f64::consts::PI * radius * radius,
            Shape::Rectangle { width, height } => width * height,
            Shape::Triangle(a, b, c) => {
                // Heron's formula.
                let s = (a + b + c) / 2.0;
                (s * (s - a) * (s - b) * (s - c)).sqrt()
            }
        }
    }

    /// Match guards and bindings: a pattern can test the shape of the value and
    /// a condition on it at once.
    pub fn describe(&self) -> String {
        match self {
            Shape::Circle { radius } if *radius > 100.0 => "a huge circle".to_string(),
            Shape::Circle { radius } => format!("a circle of radius {radius}"),
            Shape::Rectangle { width, height } if width == height => {
                format!("a {width} by {height} square")
            }
            Shape::Rectangle { .. } => "a rectangle".to_string(),
            other => format!("{other:?}"),
        }
    }
}

/// `if let` destructures a single variant without a full match.
pub fn radius_of(shape: &Shape) -> Option<f64> {
    if let Shape::Circle { radius } = shape {
        Some(*radius)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn area_per_variant() {
        assert_eq!(0.0, Shape::Point.area());
        assert_eq!(6.0, Shape::Rectangle { width: 2.0, height: 3.0 }.area());

        let circle = Shape::Circle { radius: 1.0 };
        assert!((circle.area() - std::f64::consts::PI).abs() < 1e-10);

        // A 3-4-5 right triangle has area 6.
        let triangle = Shape::Triangle(3.0, 4.0, 5.0);
        assert!((triangle.area() - 6.0).abs() < 1e-10);
    }

    #[test]
    fn guards_pick_the_first_matching_arm() {
        assert_eq!("a huge circle", Shape::Circle { radius: 500.0 }.describe());
        assert_eq!("a circle of radius 2", Shape::Circle { radius: 2.0 }.describe());
        assert_eq!(
            "a 3 by 3 square",
            Shape::Rectangle { width: 3.0, height: 3.0 }.describe()
        );
        assert_eq!("a rectangle", Shape::Rectangle { width: 3.0, height: 4.0 }.describe());
    }

    #[test]
    fn if_let_extracts_one_variant() {
        assert_eq!(Some(2.0), radius_of(&Shape::Circle { radius: 2.0 }));
        assert_eq!(None, radius_of(&Shape::Point));
    }
}
