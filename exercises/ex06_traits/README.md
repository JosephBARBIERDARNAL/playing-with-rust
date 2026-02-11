# Exercise 06: Traits

## Concept

**Traits** in Rust are like:
- Python: protocols/abstract base classes
- Java/C#: interfaces
- Haskell: typeclasses

They define shared behavior that types can implement.

## Defining and Implementing Traits

```rust
// Define a trait
trait Summarizable {
    fn summary(&self) -> String;
}

// Implement for a type
struct Article {
    title: String,
    author: String,
}

impl Summarizable for Article {
    fn summary(&self) -> String {
        format!("{} by {}", self.title, self.author)
    }
}
```

## Common Built-in Traits

### Debug - for printing with {:?}
```rust
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}
println!("{:?}", point);
```

### Clone - for duplicating values
```rust
#[derive(Clone)]
struct Data {
    values: Vec<i32>,
}
let copy = original.clone();
```

### PartialEq - for comparing with ==
```rust
#[derive(PartialEq)]
struct Person {
    name: String,
}
if person1 == person2 { ... }
```

### Display - for printing with {}
```rust
use std::fmt;

impl fmt::Display for Article {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} by {}", self.title, self.author)
    }
}
```

## Trait Bounds

Use traits to constrain generic types:

```rust
fn print_summary<T: Summarizable>(item: &T) {
    println!("{}", item.summary());
}

// Or with 'where' clause (cleaner)
fn print_summary<T>(item: &T)
where
    T: Summarizable,
{
    println!("{}", item.summary());
}

// Multiple bounds
fn compare<T: PartialEq + Debug>(a: &T, b: &T) {
    if a == b {
        println!("Same!");
    }
}
```

## Default Implementations

```rust
trait Greet {
    fn greet(&self) -> String {
        String::from("Hello!")  // Default implementation
    }
}

// Can use default or override
impl Greet for Person {}  // Uses default

impl Greet for Dog {
    fn greet(&self) -> String {
        String::from("Woof!")  // Override
    }
}
```

## Hints for this Exercise

1. Use `#[derive(...)]` for common traits instead of manual implementation
2. Trait methods can have default implementations
3. Types can implement multiple traits
4. Trait bounds make generic code more flexible
5. Use `impl Trait` for simple return types

## Example Usage

```rust
// Define a trait for data processing
trait Analyzable {
    fn mean(&self) -> f64;
    fn std_dev(&self) -> f64;

    // Default method
    fn describe(&self) -> String {
        format!("Mean: {:.2}, StdDev: {:.2}",
                self.mean(), self.std_dev())
    }
}

// Implement for Vec<f64>
impl Analyzable for Vec<f64> {
    fn mean(&self) -> f64 {
        self.iter().sum::<f64>() / self.len() as f64
    }

    fn std_dev(&self) -> f64 {
        let mean = self.mean();
        let variance = self.iter()
            .map(|x| (x - mean).powi(2))
            .sum::<f64>() / self.len() as f64;
        variance.sqrt()
    }
}

// Use it
let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
println!("{}", data.describe());
```

## For Data Scientists

Traits are powerful for:
- Defining common interfaces for different data types
- Generic algorithms that work on any type implementing a trait
- Adding methods to existing types (like pandas extending Series)

Example use case:
```rust
// Any type that can be normalized
trait Normalizable {
    fn normalize(&mut self);
}

// Works on Vec<f64>
impl Normalizable for Vec<f64> {
    fn normalize(&mut self) {
        let max = self.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
        for x in self.iter_mut() {
            *x /= max;
        }
    }
}

// Now use it generically
fn preprocess<T: Normalizable>(data: &mut T) {
    data.normalize();
}
```
