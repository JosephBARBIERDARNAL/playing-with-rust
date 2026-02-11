# Exercise 07: Generics

## Concept

**Generics** let you write code that works with multiple types. Similar to:
- Python: Type variables (`TypeVar`, `Generic[T]`)
- R: Generic functions work on any type by default
- Java/C#: Generics

Generics in Rust are **zero-cost** - no runtime overhead!

## Basic Generic Functions

```rust
// Works with any type T
fn first<T>(list: &[T]) -> &T {
    &list[0]
}

let numbers = vec![1, 2, 3];
let strings = vec!["a", "b", "c"];
first(&numbers);  // Returns &1
first(&strings);  // Returns &"a"
```

## Generic Structs

```rust
struct Point<T> {
    x: T,
    y: T,
}

let int_point = Point { x: 5, y: 10 };
let float_point = Point { x: 1.5, y: 2.5 };

// Multiple type parameters
struct Pair<T, U> {
    first: T,
    second: U,
}

let pair = Pair { first: 5, second: "hello" };
```

## Generic Enums

```rust
// Option and Result are generic!
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

## Generic Implementations

```rust
impl<T> Point<T> {
    fn new(x: T, y: T) -> Self {
        Point { x, y }
    }
}

// Can also add trait bounds
impl<T: std::fmt::Display> Point<T> {
    fn print(&self) {
        println!("({}, {})", self.x, self.y);
    }
}
```

## Combining Generics with Traits

```rust
// Function that works with any type implementing Display
fn print_it<T: std::fmt::Display>(item: T) {
    println!("{}", item);
}

// Multiple trait bounds
fn process<T>(item: T)
where
    T: Clone + Debug + PartialEq,
{
    // Can clone, debug print, and compare
}
```

## Hints for this Exercise

1. Use `<T>` to declare a generic type parameter
2. Type parameters can be anything: `T`, `U`, `Item`, etc.
3. Add trait bounds when you need specific behavior: `T: Display`
4. Use `where` clauses for complex bounds (cleaner)
5. Compiler infers types - you rarely need to specify them

## Example Usage

```rust
// Generic function for data processing
fn calculate_stats<T>(data: &[T]) -> (T, T)
where
    T: PartialOrd + Copy,
{
    let mut min = data[0];
    let mut max = data[0];

    for &item in data {
        if item < min { min = item; }
        if item > max { max = item; }
    }

    (min, max)
}

// Works with different types
let ints = vec![1, 5, 2, 9, 3];
let (min, max) = calculate_stats(&ints);

let floats = vec![1.5, 2.8, 1.2, 5.3];
let (min, max) = calculate_stats(&floats);

// Generic struct for data storage
struct DataContainer<T> {
    data: Vec<T>,
}

impl<T> DataContainer<T> {
    fn new() -> Self {
        DataContainer { data: Vec::new() }
    }

    fn add(&mut self, item: T) {
        self.data.push(item);
    }

    fn len(&self) -> usize {
        self.data.len()
    }
}

impl<T: std::fmt::Display> DataContainer<T> {
    fn print_all(&self) {
        for item in &self.data {
            println!("{}", item);
        }
    }
}

// Use it
let mut container = DataContainer::new();
container.add(42);
container.add(100);
```

## For Data Scientists

Generics are essential for:
- Writing reusable data processing functions
- Building flexible containers and data structures
- Creating type-safe APIs

Think of it as writing functions that work on DataFrame regardless of column types:

```rust
// Generic statistics function
fn summarize<T>(data: &[T]) -> Summary<T>
where
    T: Copy + PartialOrd + std::ops::Add<Output = T> + std::ops::Div<Output = T>,
{
    // Calculate mean, median, etc.
}

// Now works with i32, f64, or any numeric type!
```

The key insight: **Write once, use everywhere** - but with compile-time type safety!
