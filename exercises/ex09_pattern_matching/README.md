# Exercise 09: Advanced Pattern Matching

## Concept

Pattern matching in Rust goes far beyond basic `match` on enums. It's a powerful tool for destructuring and controlling flow.

## Basic Match (Review)

```rust
let x = 5;
match x {
    1 => println!("one"),
    2 | 3 => println!("two or three"),  // Multiple patterns
    4..=10 => println!("four through ten"),  // Range
    _ => println!("something else"),  // Catch-all
}
```

## Destructuring

### Tuples
```rust
let point = (3, 5);
match point {
    (0, 0) => println!("Origin"),
    (x, 0) => println!("On x-axis at {}", x),
    (0, y) => println!("On y-axis at {}", y),
    (x, y) => println!("At ({}, {})", x, y),
}
```

### Structs
```rust
struct Point {
    x: i32,
    y: i32,
}

let p = Point { x: 0, y: 7 };
match p {
    Point { x: 0, y } => println!("On y-axis at y={}", y),
    Point { x, y: 0 } => println!("On x-axis at x={}", x),
    Point { x, y } => println!("At ({}, {})", x, y),
}

// Can also use ..
match p {
    Point { x: 0, .. } => println!("x is zero"),
    _ => println!("x is not zero"),
}
```

### Enums
```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

match msg {
    Message::Quit => println!("Quit"),
    Message::Move { x, y } => println!("Move to ({}, {})", x, y),
    Message::Write(text) => println!("Text: {}", text),
    Message::ChangeColor(r, g, b) => println!("RGB: ({}, {}, {})", r, g, b),
}
```

## Guards

Add extra conditions with `if`:

```rust
let num = Some(4);
match num {
    Some(x) if x < 5 => println!("Less than 5: {}", x),
    Some(x) => println!("Greater or equal to 5: {}", x),
    None => println!("No value"),
}
```

## @ Bindings

Bind values while matching:

```rust
match age {
    n @ 1..=12 => println!("Child age: {}", n),
    n @ 13..=19 => println!("Teen age: {}", n),
    n => println!("Adult age: {}", n),
}
```

## if let and while let

Shorter syntax when you only care about one pattern:

```rust
// Instead of match with one arm
if let Some(value) = option {
    println!("Got: {}", value);
}

// Loop while pattern matches
let mut stack = vec![1, 2, 3];
while let Some(top) = stack.pop() {
    println!("{}", top);
}
```

## let Destructuring

You can destructure in `let` statements:

```rust
let (x, y) = (1, 2);

let Point { x, y } = point;

// With annotations
let Point { x: new_x, y: new_y } = point;
```

## Hints for this Exercise

1. Use `match` when you need exhaustive handling
2. Use `if let` when you only care about one case
3. Guards (`if` conditions) make matches more specific
4. `@` bindings let you capture while matching
5. Ranges work with any orderable type: `1..=100`

## Example Usage

```rust
// Destructuring in function parameters
fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}

let point = (3, 5);
print_coordinates(&point);

// Complex nested matching
enum Data {
    Single(i32),
    Pair(i32, i32),
    Multiple(Vec<i32>),
}

fn process(data: Data) {
    match data {
        Data::Single(x) if x > 0 => println!("Positive single: {}", x),
        Data::Single(x) => println!("Non-positive: {}", x),
        Data::Pair(x, y) if x == y => println!("Equal pair: {}", x),
        Data::Pair(x, y) => println!("Different: {} and {}", x, y),
        Data::Multiple(ref v) if v.is_empty() => println!("Empty vector"),
        Data::Multiple(v) => {
            println!("Vector with {} elements", v.len());
            if let Some(&first) = v.first() {
                println!("First element: {}", first);
            }
        }
    }
}

// Pattern matching in iterators
let pairs = vec![(1, 2), (3, 4), (5, 6)];
for (x, y) in pairs {
    println!("{} + {} = {}", x, y, x + y);
}

// Filtering with pattern matching
let options = vec![Some(1), None, Some(3), None, Some(5)];
let values: Vec<i32> = options
    .into_iter()
    .filter_map(|x| match x {
        Some(n) if n > 2 => Some(n * 2),
        _ => None,
    })
    .collect();
```

## For Data Scientists

Pattern matching is excellent for:
- **Data cleaning**: Handle missing values elegantly
- **Data transformation**: Match and transform in one step
- **Type narrowing**: Safely extract values from enums
- **Conditional logic**: Replace complex if-else chains

Think of it like:
- Python: `match` statement (3.10+) but more powerful
- R: `switch()` but with destructuring
- SQL: `CASE WHEN` but compile-time checked

Example for data work:
```rust
// Handle different data scenarios
match (mean, std_dev) {
    (m, s) if s < 0.01 => println!("Constant data"),
    (m, s) if s > 100.0 => println!("High variance"),
    (m, s) => println!("Normal: mean={}, std={}", m, s),
}

// Clean missing data
let cleaned: Vec<f64> = raw_data
    .into_iter()
    .filter_map(|x| match x {
        Some(v) if v.is_finite() => Some(v),
        _ => None,
    })
    .collect();
```
