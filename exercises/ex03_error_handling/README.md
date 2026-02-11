# Exercise 03: Error Handling

## Concept

Rust doesn't have exceptions like Python's try/except. Instead, it uses:
- **`Option<T>`** - for values that might not exist (like Python's `None`)
- **`Result<T, E>`** - for operations that might fail (like try/except)

## Option<T>

Used when a value may or may not exist:

```rust
enum Option<T> {
    Some(T),    // Has a value
    None,       // No value
}
```

Python equivalent:
```python
def find_item(lst, target):
    try:
        return lst.index(target)
    except ValueError:
        return None  # or return -1
```

Rust:
```rust
fn find_item(vec: &Vec<i32>, target: i32) -> Option<usize> {
    vec.iter().position(|&x| x == target)
}
```

## Result<T, E>

Used for operations that can fail with an error:

```rust
enum Result<T, E> {
    Ok(T),      // Success with value T
    Err(E),     // Error with error type E
}
```

Python:
```python
def divide(a, b):
    try:
        return a / b
    except ZeroDivisionError as e:
        raise e  # or handle it
```

Rust:
```rust
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("Division by zero"))
    } else {
        Ok(a / b)
    }
}
```

## Working with Option and Result

### Unwrapping (use carefully!)
```rust
let x: Option<i32> = Some(5);
let val = x.unwrap();  // Gets 5, panics if None

// Better: provide default
let val = x.unwrap_or(0);  // Gets 5, or 0 if None
```

### Pattern Matching (preferred)
```rust
match x {
    Some(val) => println!("Got: {}", val),
    None => println!("No value"),
}
```

### The `?` Operator
Propagates errors up the call stack (like re-raising in Python):

```rust
fn read_and_parse() -> Result<i32, String> {
    let contents = read_file()?;  // If error, return early
    let num = parse_number(&contents)?;  // If error, return early
    Ok(num)
}
```

## Hints for this Exercise

1. Use `Option` when something might not exist (finding items, dict lookups)
2. Use `Result` when an operation might fail (I/O, parsing, division)
3. Pattern matching with `match` is the safest way to handle these
4. `unwrap()` panics on None/Err - only use when you're SURE or prototyping
5. The `?` operator makes error handling clean and readable

## Example Usage

```rust
// Option example
fn find_max(numbers: &[i32]) -> Option<i32> {
    if numbers.is_empty() {
        None
    } else {
        Some(*numbers.iter().max().unwrap())
    }
}

match find_max(&[1, 5, 3]) {
    Some(max) => println!("Max: {}", max),
    None => println!("Empty list"),
}

// Result example
fn parse_age(input: &str) -> Result<u32, String> {
    match input.parse::<u32>() {
        Ok(age) if age <= 120 => Ok(age),
        Ok(_) => Err(String::from("Age too high")),
        Err(_) => Err(String::from("Not a number")),
    }
}

match parse_age("25") {
    Ok(age) => println!("Age: {}", age),
    Err(e) => println!("Error: {}", e),
}
```

## For Data Scientists

- `Option` is like `None` in Python or `NA` in R
- `Result` is like try/except but handled at compile time
- This prevents runtime errors and makes error handling explicit
- Think: `Option` = missing data, `Result` = failed computation
