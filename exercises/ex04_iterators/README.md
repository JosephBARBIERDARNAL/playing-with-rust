# Exercise 04: Iterators

## Concept

Iterators in Rust are similar to Python generators or R's apply family, but they're **zero-cost abstractions** - as fast as hand-written loops! They follow a lazy evaluation model.

## Iterator Basics

```rust
let v = vec![1, 2, 3, 4, 5];
let iter = v.iter();  // Creates an iterator
```

### Three ways to create iterators:
- `.iter()` - borrows each element (`&T`)
- `.iter_mut()` - mutably borrows each element (`&mut T`)
- `.into_iter()` - takes ownership, consumes the collection

## Common Iterator Methods (Similar to Python/Pandas)

### Map (transform)
Python: `[x * 2 for x in numbers]`
```rust
let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
```

### Filter (select)
Python: `[x for x in numbers if x > 10]`
```rust
let filtered: Vec<&i32> = numbers.iter().filter(|&x| x > &10).collect();
```

### Reduce/Fold
Python: `sum(numbers)`
```rust
let sum: i32 = numbers.iter().sum();
let product: i32 = numbers.iter().product();
let custom = numbers.iter().fold(0, |acc, x| acc + x);
```

### Other useful methods
```rust
// Take first n elements
numbers.iter().take(3).collect()

// Skip first n elements
numbers.iter().skip(2).collect()

// Chain iterators
iter1.chain(iter2)

// Enumerate (like Python's enumerate)
for (i, val) in numbers.iter().enumerate() {
    println!("{}: {}", i, val);
}

// Find first matching
numbers.iter().find(|&&x| x > 10)

// All/Any
numbers.iter().all(|&x| x > 0)
numbers.iter().any(|&x| x > 100)
```

## Chaining Operations (Lazy Evaluation)

```rust
let result: Vec<i32> = numbers
    .iter()
    .filter(|&&x| x % 2 == 0)  // Only even numbers
    .map(|&x| x * x)           // Square them
    .take(5)                   // Take first 5
    .collect();                // Collect into Vec
```

Nothing is computed until `.collect()` or another consumer!

## Hints for this Exercise

1. `.iter()` gives you references (`&T`), use `*` or patterns to dereference
2. `.collect()` is needed to turn an iterator back into a collection
3. You must specify the type when collecting: `.collect::<Vec<_>>()`
4. Closures in iterators use `|param| expression` syntax
5. Chain operations for readable data pipelines

## Example Usage

```rust
fn main() {
    let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // Filter and map (like pandas/dplyr)
    let result: Vec<i32> = data
        .iter()
        .filter(|&&x| x % 2 == 0)  // Even numbers
        .map(|&x| x * x)           // Square them
        .collect();
    println!("{:?}", result);  // [4, 16, 36, 64, 100]

    // Aggregations
    let sum: i32 = data.iter().sum();
    let mean = sum as f64 / data.len() as f64;

    // Find operations
    let first_big = data.iter().find(|&&x| x > 5);
    match first_big {
        Some(&val) => println!("Found: {}", val),
        None => println!("Not found"),
    }
}
```

## For Data Scientists

Think of iterators as:
- pandas: `df.query().assign().pipe()`
- R/dplyr: `data %>% filter() %>% mutate()`
- NumPy: array operations but lazy
- Python: generator expressions and itertools

Key advantage: **composable and efficient**. No intermediate allocations!

```rust
// This creates NO intermediate vectors!
data.iter()
    .filter(|&&x| x > 10)
    .map(|&x| x * 2)
    .sum()  // Single pass through data
```
