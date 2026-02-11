# Exercise 08: Collections (HashMap & HashSet)

## Concept

Beyond `Vec`, Rust has other useful collections for data work:
- **`HashMap<K, V>`** - Like Python's `dict` or R's named list
- **`HashSet<T>`** - Like Python's `set` or R's unique values

## HashMap Basics

```rust
use std::collections::HashMap;

// Create a new HashMap
let mut scores = HashMap::new();

// Insert key-value pairs
scores.insert(String::from("Alice"), 95);
scores.insert(String::from("Bob"), 87);

// Get values
match scores.get("Alice") {
    Some(&score) => println!("Score: {}", score),
    None => println!("Not found"),
}

// Update values
scores.insert(String::from("Alice"), 98);  // Overwrites

// Insert only if key doesn't exist
scores.entry(String::from("Carol")).or_insert(90);

// Iterate
for (name, score) in &scores {
    println!("{}: {}", name, score);
}
```

## Common HashMap Patterns

### Counting occurrences (like pandas value_counts)
```rust
let text = "hello world hello rust";
let mut word_count = HashMap::new();

for word in text.split_whitespace() {
    let count = word_count.entry(word).or_insert(0);
    *count += 1;
}
// {"hello": 2, "world": 1, "rust": 1}
```

### Group by key (like pandas groupby)
```rust
let data = vec![("A", 10), ("B", 20), ("A", 30)];
let mut grouped: HashMap<&str, Vec<i32>> = HashMap::new();

for (key, val) in data {
    grouped.entry(key).or_insert(Vec::new()).push(val);
}
// {"A": [10, 30], "B": [20]}
```

### Update with default
```rust
// Update or insert default
let count = map.entry("key").or_insert(0);
*count += 1;

// Update with function
map.entry("key")
    .and_modify(|e| *e += 1)
    .or_insert(1);
```

## HashSet Basics

```rust
use std::collections::HashSet;

// Create a set
let mut numbers = HashSet::new();
numbers.insert(1);
numbers.insert(2);
numbers.insert(1);  // Duplicate, ignored

// Check membership
if numbers.contains(&1) {
    println!("Contains 1");
}

// Set operations
let set1: HashSet<_> = vec![1, 2, 3].into_iter().collect();
let set2: HashSet<_> = vec![2, 3, 4].into_iter().collect();

let union: HashSet<_> = set1.union(&set2).cloned().collect();
let intersection: HashSet<_> = set1.intersection(&set2).cloned().collect();
let difference: HashSet<_> = set1.difference(&set2).cloned().collect();
```

## Hints for this Exercise

1. Need to `use std::collections::HashMap;` and `use std::collections::HashSet;`
2. `.get()` returns `Option<&V>` - use pattern matching or `.unwrap()`
3. `.entry()` API is powerful for conditional insertion/updates
4. To iterate and modify values, get mutable reference: `for (k, v) in &mut map`
5. HashSet is useful for unique values and set operations

## Example Usage

```rust
use std::collections::HashMap;

// Building a frequency map (like value_counts in pandas)
fn count_frequency(items: &[i32]) -> HashMap<i32, usize> {
    let mut freq = HashMap::new();
    for &item in items {
        *freq.entry(item).or_insert(0) += 1;
    }
    freq
}

let data = vec![1, 2, 2, 3, 3, 3, 4];
let freq = count_frequency(&data);
for (num, count) in &freq {
    println!("{}: {}", num, count);
}

// Finding unique values (like pandas unique)
use std::collections::HashSet;

fn get_unique(items: &[i32]) -> Vec<i32> {
    let set: HashSet<_> = items.iter().cloned().collect();
    set.into_iter().collect()
}

// Group by and aggregate (like SQL GROUP BY)
fn group_and_sum(data: Vec<(&str, i32)>) -> HashMap<&str, i32> {
    let mut result = HashMap::new();
    for (key, val) in data {
        *result.entry(key).or_insert(0) += val;
    }
    result
}

let sales = vec![("Alice", 100), ("Bob", 150), ("Alice", 200)];
let totals = group_and_sum(sales);
// {"Alice": 300, "Bob": 150}
```

## For Data Scientists

HashMap and HashSet are essential for:
- **Grouping data** - Like pandas groupby or R's aggregate
- **Counting frequencies** - Like value_counts()
- **Finding unique values** - Like unique() or distinct()
- **Lookups** - Fast O(1) average case
- **Set operations** - Union, intersection, difference

Common patterns:
```rust
// Pivot table
let mut pivot: HashMap<(String, String), i32> = HashMap::new();
pivot.insert(("A".to_string(), "X".to_string()), 10);

// Index by key
let mut index: HashMap<String, Vec<Record>> = HashMap::new();

// Cache/memoization
let mut cache: HashMap<String, ComputedResult> = HashMap::new();
```
