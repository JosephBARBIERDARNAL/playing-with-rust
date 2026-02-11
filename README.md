# Rust Learning Path for Data Scientists

Welcome! This is a structured learning path to help you learn Rust, designed specifically for someone with Python/R and web development experience.

## Getting Started

```bash
# Run the interactive exercise runner
cargo run

# Or run directly
./target/debug/playing-with-rust
```

## Learning Path

The exercises are designed to build on each other, taking you from Rust fundamentals to advanced concepts:

### Foundation (Start Here!)
1. **Ownership** - Rust's unique memory management system
2. **Borrowing & References** - Safe references without ownership transfer
3. **Error Handling** - `Option` and `Result` types for safe error handling

### Core Skills
4. **Iterators** - Zero-cost abstractions for data processing
5. **Closures** - Anonymous functions with environment capture
6. **Traits** - Shared behavior (like interfaces/protocols)
7. **Generics** - Write code that works with multiple types

### Data Structures & Advanced Topics
8. **Collections** - HashMap and HashSet for real-world data work
9. **Pattern Matching** - Powerful control flow and destructuring
10. **Lifetimes** - Advanced reference validation

## How to Use This

Each exercise has two files:

### `README.md`
- **Concept explanation** - What is this feature?
- **Comparisons to Python/R** - How does it relate to what you know?
- **Example usage** - See it in action
- **Hints** - Tips for completing the exercise

### `mod.rs`
- **Code challenges** - Fill in the TODOs
- **Progressive difficulty** - Start simple, build up
- **Tests** - Run `cargo test` to check your solutions

## Recommended Workflow

1. **Read the README first** - Understand the concept before coding
2. **Work through challenges in order** - They build on each other
3. **Uncomment code as you go** - The exercises have commented-out code to enable
4. **Run frequently** - Use `cargo run` and select your exercise number
5. **Check with tests** - `cargo test` to validate your solutions
6. **Experiment** - Modify the code, try things out!

## Key Differences from Python/R

| Concept | Python/R | Rust |
|---------|----------|------|
| Memory | Garbage collected | Ownership system |
| Types | Dynamic | Static, inferred |
| Mutability | Default mutable | Default immutable |
| Null | `None`/`NULL` | `Option<T>` enum |
| Errors | Exceptions | `Result<T, E>` enum |
| Performance | Interpreted/JIT | Compiled, zero-cost abstractions |

## Tips for Data Scientists

- **Think in pipelines** - Rust iterators are like pandas chains or dplyr pipes
- **Embrace the compiler** - Error messages are helpful, not scary!
- **Immutability by default** - This prevents many data bugs
- **Zero-cost abstractions** - High-level code compiles to fast machine code
- **No runtime overhead** - Unlike Python, Rust has no GIL or GC pauses

## Getting Help

- **Compiler errors** - Read them carefully, they're usually clear
- **Each README** - Has hints and examples
- **The Rust Book** - https://doc.rust-lang.org/book/
- **Rust by Example** - https://doc.rust-lang.org/rust-by-example/

## Testing Your Solutions

```bash
# Run a specific exercise
cargo run
# Then enter the number (1-10)

# Run all exercises
cargo run
# Then type 'all'

# Run tests for all exercises
cargo test

# Run tests for a specific module
cargo test ex01_ownership
```

## Project Structure

```
.
├── main.rs              # Interactive exercise runner
├── exercises/
│   ├── ex01_ownership/
│   │   ├── README.md    # Concept explanation
│   │   └── mod.rs       # Exercise code
│   ├── ex02_borrowing_references/
│   │   ├── README.md
│   │   └── mod.rs
│   └── ...
└── README.md            # This file
```

## Next Steps After Completing

Once you finish these exercises, you'll be ready to:

1. **Build CLI tools** - Use `clap` for argument parsing
2. **Work with data** - Try `polars` (like pandas but in Rust)
3. **Web development** - Explore `axum` or `actix-web`
4. **Data science** - Check out `ndarray`, `linfa` for ML
5. **Real projects** - Apply your knowledge to actual problems!

## Why Learn Rust as a Data Scientist?

- **Performance** - 100x faster than Python for some tasks
- **Safety** - Catch bugs at compile time, not in production
- **Concurrency** - Parallelism without data races
- **Deployment** - Single binary, no dependencies
- **Growing ecosystem** - Tools like Polars are gaining traction

Happy learning! 🦀

---

*Remember: The compiler is your friend. It's strict because it wants to help you write correct code.*
