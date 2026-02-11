# Exercise 02: Borrowing & References

## Concept

**Borrowing** lets you refer to data without taking ownership. This is like passing a pointer in C, or how references work in Python - but with strict rules enforced at compile time.

### Borrowing Rules:
1. You can have **many immutable references** (`&T`) OR **one mutable reference** (`&mut T`)
2. References must always be valid (no dangling pointers!)
3. These rules prevent data races at compile time

## Python vs Rust Analogy

In Python, you might pass a list to a function:
```python
def add_item(lst):
    lst.append(4)  # Modifies original

my_list = [1, 2, 3]
add_item(my_list)  # my_list is now [1, 2, 3, 4]
```

In Rust, you explicitly borrow:
```rust
fn add_item(lst: &mut Vec<i32>) {
    lst.push(4);  // Modifies original via mutable reference
}

let mut my_vec = vec![1, 2, 3];
add_item(&mut my_vec);  // Borrow mutably
// my_vec is now [1, 2, 3, 4]
```

## Immutable vs Mutable References

```rust
// Immutable borrowing (reading)
let s = String::from("hello");
let r1 = &s;  // OK
let r2 = &s;  // OK - multiple immutable references allowed
println!("{} {}", r1, r2);

// Mutable borrowing (writing)
let mut s = String::from("hello");
let r1 = &mut s;  // OK
// let r2 = &mut s;  // ERROR: only one mutable reference at a time!
r1.push_str(" world");
```

## Hints for this Exercise

1. Use `&` to create an immutable reference, `&mut` for a mutable one
2. The variable being borrowed must be `mut` to borrow it mutably
3. You can't have a mutable reference while immutable references exist
4. Think of `&` as "read-only access" and `&mut` as "read-write access"

## Example Usage

```rust
// Reading data without taking ownership
fn calculate_length(s: &String) -> usize {
    s.len()
}  // s goes out of scope but doesn't drop the String (just borrowed)

fn main() {
    let my_string = String::from("hello");
    let len = calculate_length(&my_string);  // Borrow with &
    println!("Length: {}, String: {}", len, my_string);  // Still valid!
}

// Modifying data without taking ownership
fn append_world(s: &mut String) {
    s.push_str(" world");
}

fn main_mut() {
    let mut my_string = String::from("hello");
    append_world(&mut my_string);
    println!("{}", my_string);  // "hello world"
}
```

## For Data Scientists

Think of references like views in NumPy or pandas:
- `&T` is like a read-only view (won't modify original)
- `&mut T` is like a mutable view (can modify in-place)
- Rust enforces at compile time that you can't have multiple mutable views simultaneously, preventing bugs
