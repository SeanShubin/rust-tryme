//! Entry point for the prototype version.
//!
//! There is nothing to wire, because there is nothing to choose: `run` goes to
//! the operating system itself.
//!
//! ```text
//! cargo run --bin prototype -- target.txt
//! ```

fn main() {
    domain::prototype::run();
}
