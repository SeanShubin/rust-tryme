# rust-tryme

Small, self-contained experiments in Rust language features — the Rust
counterpart to `kotlin-tryme`.

Each module in the `domain` crate isolates one idea and pins it down with
`#[test]` functions, so `cargo test` is the fastest way to check whether an
assumption about the language holds. The `console` crate runs the handful of
experiments where watching the output is the point.

## Installing rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"

Verify with:
rustc --version
cargo --version

## Layout

| Crate | What it holds |
|---|---|
| `crates/domain` | the experiments, one module per idea, each with its own tests |
| `crates/console` | binaries: `console` runs the ones worth watching, `prototype` and `injected` are the two entry points for the paired program |

Current experiments: ownership and borrowing, enums and pattern matching,
traits and dispatch, iterators and closures, errors as values, derive macros
via serde, type-driven generics via rand, and one program written twice --
once as a prototype and once with its dependencies injected.

## The same program, two ways

`domain::prototype` and `domain::dependency_injection` are the same program:
read the file named by the first argument, greet whoever it names, say how long
it took. They print the same thing.

| | `prototype` | `dependency_injection` |
|---|---|---|
| Lines | about ten | about two hundred, counting tests |
| Clock, files, stdout | reached for directly | passed in as contracts |
| Failures | `expect`, so a panic | an `ApplicationError` the caller can act on |
| Tests | none are possible | the clock, the file system and the output are all fixtures |

The prototype is not sloppy, it is early -- it is how you find out whether an
idea works at all. The injected version is what that idea costs once it has to
keep working. The interesting part is the seam: `ApplicationBehavior` is handed
a `ClockContract`, a `FilesContract` and an emit function, so a test can hand it
a clock that jumps from 1000ms to 1250ms and assert on `250 milliseconds`
instead of hoping the machine was slow enough to notice.

`Dependencies` is the only type that names both a contract and a real effect,
which is what keeps that choice in one place.

```bash
echo -n world > target.txt
cargo run --bin prototype -- target.txt
cargo run --bin injected   -- target.txt
```

## Running

```bash
./scripts/test.sh      # cargo test --workspace
./scripts/build.sh     # cargo build --workspace
./scripts/doc.sh       # cargo doc --workspace --no-deps
cargo run --bin console
cargo run --bin prototype -- target.txt
cargo run --bin injected  -- target.txt
```

The `prototype` and `injected` binaries live in `crates/console/src/bin/`,
where cargo finds them without a `[[bin]]` entry -- so regenerating the build
files does not drop them.

PowerShell equivalents (`scripts/*.ps1`) are generated alongside each script.

## Regenerating

The build files are generated from `project-specification.json` by
[project-generator](https://github.com/SeanShubin/project-generator):

```bash
java -jar ../project-generator/console/target/project-generator-console.jar
```

Regeneration rewrites `Cargo.toml` at the root and in each crate, and creates
`src/lib.rs` or `src/main.rs` only when absent — so hand-written source is never
overwritten. To add a crate, add it to `modules` in the specification and
regenerate.
