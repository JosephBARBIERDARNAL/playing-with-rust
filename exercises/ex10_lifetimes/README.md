# Exercise 10: Lifetimes

## Concept

**Lifetimes** are Rust's way of ensuring references are valid. The compiler tracks how long references live to prevent dangling pointers.

Most of the time, lifetimes are implicit. You only need explicit lifetime annotations when the compiler can't figure out the relationships.

## Why Lifetimes?

Python/R don't have this because they use garbage collection. In Rust, we need to prove references won't outlive the data they point to.

```rust
// This won't compile - dangling reference!
fn broken() -> &String {
    let s = String::from("hello");
    &s  // ERROR: s is dropped at end of function!
}
```

## Lifetime Syntax

Lifetimes are denoted with `'a`, `'b`, etc. (pronounce "tick a"):

```rust
// Basic lifetime annotation
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

This says: "The returned reference will live as long as both input references."

## Common Patterns

### Multiple references
```rust
// Return value has same lifetime as inputs
fn first_word<'a>(s: &'a str) -> &'a str {
    s.split_whitespace().next().unwrap()
}
```

### Structs with references
```rust
struct Excerpt<'a> {
    part: &'a str,
}

impl<'a> Excerpt<'a> {
    fn announce(&self) -> &str {
        self.part
    }
}

let text = String::from("Call me Ishmael.");
let excerpt = Excerpt { part: &text };
```

### Multiple lifetimes
```rust
// x and y can have different lifetimes
// Return is tied to x's lifetime
fn first<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
    x
}
```

## Lifetime Elision Rules

The compiler can infer lifetimes in common cases:

1. Each reference parameter gets its own lifetime
2. If one input lifetime, it's assigned to all outputs
3. If `&self` or `&mut self`, its lifetime is assigned to outputs

Examples where you DON'T need annotations:
```rust
fn first_word(s: &str) -> &str { ... }  // Rule 2
fn self_method(&self) -> &str { ... }   // Rule 3
```

## Hints for this Exercise

1. Start without lifetime annotations - add only when compiler complains
2. Use the same lifetime `'a` when references should have the same scope
3. Lifetimes are about relationships, not actual duration
4. Read compiler errors carefully - they're usually helpful!
5. Most functions don't need explicit lifetimes

## Example Usage

```rust
// Find the longer of two strings
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("long string");
    let result;

    {
        let string2 = String::from("short");
        result = longest(&string1, &string2);
        println!("Longer: {}", result);  // OK, both live here
    }
    // Can't use result here - string2 was dropped!
}

// Struct holding a reference
struct DataView<'a> {
    data: &'a [i32],
    start: usize,
    end: usize,
}

impl<'a> DataView<'a> {
    fn new(data: &'a [i32], start: usize, end: usize) -> Self {
        DataView { data, start, end }
    }

    fn slice(&self) -> &[i32] {
        &self.data[self.start..self.end]
    }

    fn len(&self) -> usize {
        self.end - self.start
    }
}

fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    let view = DataView::new(&numbers, 1, 4);
    println!("{:?}", view.slice());  // [2, 3, 4]
}
```

## For Data Scientists

Lifetimes prevent common bugs:
- Using a reference after the data is freed
- Holding references while modifying data
- Passing references that outlive their data

Think of lifetimes like:
- Ensuring a numpy view doesn't outlive the array
- Making sure DataFrame references stay valid
- Preventing use-after-free bugs automatically

You'll encounter lifetimes when:
1. Creating structs that hold references (views, slices)
2. Returning references from functions
3. Complex borrowing patterns

The good news: most code doesn't need explicit lifetimes! The compiler is smart.

## When You Need Lifetimes

You'll typically need to annotate when:
- Functions take multiple references and return a reference
- Structs hold references (always need lifetime)
- Complex generic code with references

Start simple, add lifetimes only when compiler asks!
