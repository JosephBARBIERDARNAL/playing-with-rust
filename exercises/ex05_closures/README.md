# Exercise 05: Closures

## Concept

Closures in Rust are like lambda functions in Python or anonymous functions in R, but with more powerful **capturing** semantics. They can capture variables from their environment.

## Basic Syntax

```rust
// Python
lambda x: x + 1

# Rust
|x| x + 1
```

```rust
let add_one = |x| x + 1;
let result = add_one(5);  // 6

// With type annotations (usually inferred)
let add = |x: i32, y: i32| -> i32 { x + y };
```

## Capturing Environment

Unlike Python lambdas (which can only capture), Rust closures can:
1. **Borrow immutably** (`Fn`) - like read-only
2. **Borrow mutably** (`FnMut`) - like read-write
3. **Take ownership** (`FnOnce`) - consumes captured variables

```rust
let x = 10;

// Immutable borrow
let print_x = || println!("{}", x);
print_x();  // Can call multiple times

// Mutable borrow
let mut count = 0;
let mut increment = || {
    count += 1;
    count
};
println!("{}", increment());  // 1
println!("{}", increment());  // 2

// Takes ownership
let data = vec![1, 2, 3];
let consume = || {
    println!("{:?}", data);
    // data is moved into closure
};
consume();
// consume();  // ERROR: can only call once!
```

## Closure Traits

Rust has three closure traits:

1. **`Fn`** - Borrows values immutably
   - Can be called multiple times
   - Most flexible

2. **`FnMut`** - Borrows values mutably
   - Can be called multiple times
   - Can modify captured variables

3. **`FnOnce`** - Takes ownership
   - Can only be called once
   - Consumes captured variables

## Using Closures with Functions

```rust
fn apply_operation<F>(x: i32, op: F) -> i32
where
    F: Fn(i32) -> i32,
{
    op(x)
}

let result = apply_operation(5, |x| x * 2);  // 10
```

## Hints for this Exercise

1. Closures capture variables from their environment automatically
2. Use `move` keyword to force taking ownership: `move |x| { ... }`
3. Closures are commonly used with iterators: `.map()`, `.filter()`, etc.
4. When returning a closure, use `Box<dyn Fn...>` or `impl Fn...`
5. Compiler infers closure traits based on how you use captured variables

## Example Usage

```rust
fn main() {
    // Simple closure
    let square = |x: i32| x * x;
    println!("{}", square(4));  // 16

    // Capturing environment
    let factor = 10;
    let multiply = |x| x * factor;
    println!("{}", multiply(5));  // 50

    // With iterators (common pattern)
    let numbers = vec![1, 2, 3, 4, 5];
    let threshold = 3;
    let filtered: Vec<i32> = numbers
        .into_iter()
        .filter(|&x| x > threshold)  // Closure captures threshold
        .collect();
    println!("{:?}", filtered);  // [4, 5]

    // Mutable capture
    let mut sum = 0;
    let mut accumulate = |x| {
        sum += x;
        sum
    };
    println!("{}", accumulate(5));   // 5
    println!("{}", accumulate(10));  // 15
}

// Returning closures
fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
    move |x| x + n
}

let add_five = make_adder(5);
println!("{}", add_five(10));  // 15
```

## For Data Scientists

Think of closures as:
- Python: lambdas but more powerful
- R: anonymous functions
- pandas: `df.apply(lambda x: ...)`

Common use cases in data work:
```rust
// Transforming data
data.iter().map(|x| x * 2)

// Filtering
data.iter().filter(|&x| x > threshold)

// Custom operations
data.iter().fold(0, |acc, x| acc + x)

// Parameterized operations
fn apply_scaling(data: Vec<f64>, factor: f64) -> Vec<f64> {
    data.iter().map(|&x| x * factor).collect()
}
```

Key difference from Python: Rust captures are explicit about ownership, preventing bugs!
