# Exercise 01: Ownership

## Concept

**Ownership** is Rust's most unique feature. Unlike Python where the garbage collector manages memory, Rust uses ownership rules enforced at compile time.

### Three Rules of Ownership:
1. Each value in Rust has a variable that's called its **owner**
2. There can only be **one owner** at a time
3. When the owner goes out of scope, the value is dropped (memory freed)

## Key Differences from Python/R

In Python:
```python
x = [1, 2, 3]
y = x  # Both x and y reference the same list
y.append(4)  # Modifies the list for both
```

In Rust:
```rust
let x = vec![1, 2, 3];
let y = x;  // x is MOVED to y, x no longer valid!
// println!("{:?}", x);  // ERROR: x was moved
```

## Move vs Copy

- **Copy types** (integers, floats, bools, chars): Small types stored on the stack, copied automatically
- **Move types** (String, Vec, HashMap): Heap-allocated, ownership transfers

## Hints for this Exercise

1. If you try to use a variable after it's been moved, you'll get a compile error
2. You can explicitly `.clone()` data to create a copy (expensive for large data)
3. Think about who "owns" each piece of data at any given time
4. Functions also take ownership unless you pass a reference (next exercise!)

## Example Usage

```rust
fn main() {
    // Copy types (stored on stack)
    let a = 5;
    let b = a;  // a is copied, both valid
    println!("{} {}", a, b);  // Works fine

    // Move types (heap allocated)
    let s1 = String::from("hello");
    let s2 = s1;  // s1 is moved to s2
    // println!("{}", s1);  // ERROR!
    println!("{}", s2);  // OK

    // Cloning creates a deep copy
    let s3 = String::from("world");
    let s4 = s3.clone();
    println!("{} {}", s3, s4);  // Both valid
}

fn takes_ownership(s: String) {
    println!("{}", s);
}  // s is dropped here

fn main_example() {
    let my_string = String::from("hello");
    takes_ownership(my_string);
    // println!("{}", my_string);  // ERROR: was moved into function
}
```

## For Data Scientists

Think of ownership like having exclusive write access to a file. In Python, multiple variables can reference the same object (like multiple readers). In Rust, ownership ensures only one "writer" at a time, preventing data races.
