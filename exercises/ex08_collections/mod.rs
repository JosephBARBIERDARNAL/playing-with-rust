// Exercise 08: Collections (HashMap & HashSet)

use std::collections::{HashMap, HashSet};

pub fn exercise() {
    println!("\n=== Exercise 08: Collections ===");

    challenge_1();
    challenge_2();
    challenge_3();
    challenge_4();
}

fn challenge_1() {
    // TODO: Create a HashMap to store student scores
    // let mut scores = HashMap::new();
    // scores.insert("Alice", 95);
    // scores.insert("Bob", 87);
    // scores.insert("Carol", 92);

    // Print Alice's score
    // Update Bob's score to 90
    // Print all scores
}

fn challenge_2() {
    // TODO: Count word frequencies (like pandas value_counts)
    let text = "the quick brown fox jumps over the lazy dog the fox";

    // Create a HashMap to count each word
    // let mut word_count = HashMap::new();

    // Split text and count each word
    // for word in text.split_whitespace() {
    //     ???
    // }

    // Print the counts
}

fn challenge_3() {
    // TODO: Group by key and collect values
    // Given: vec![("A", 10), ("B", 20), ("A", 30), ("B", 40)]
    // Create: HashMap where "A" -> vec![10, 30], "B" -> vec![20, 40]

    let data = vec![("A", 10), ("B", 20), ("A", 30), ("B", 40)];
    // let mut grouped: HashMap<&str, Vec<i32>> = HashMap::new();

    // for (key, value) in data {
    //     ???
    // }

    // println!("{:?}", grouped);
}

fn challenge_4() {
    // TODO: Use HashSet to find unique values
    let numbers = vec![1, 2, 2, 3, 4, 3, 5, 1, 6];

    // let unique: HashSet<i32> = ???

    // println!("Unique numbers: {:?}", unique);
}

// Advanced: Implement a function to calculate group statistics
fn group_statistics(data: Vec<(&str, f64)>) -> HashMap<&str, (f64, f64)> {
    // TODO: For each group (key), calculate (sum, count)
    // Input: vec![("A", 10.0), ("B", 20.0), ("A", 30.0)]
    // Output: {"A": (40.0, 2.0), "B": (20.0, 1.0)}
    HashMap::new()
}

fn challenge_5() {
    let data = vec![("A", 10.0), ("B", 20.0), ("A", 30.0), ("B", 25.0)];
    let stats = group_statistics(data);

    // for (key, (sum, count)) in stats {
    //     let mean = sum / count;
    //     println!("{}: sum={}, mean={:.2}", key, sum, mean);
    // }
}

// Advanced: Set operations
fn challenge_6() {
    // TODO: Use HashSet for set operations
    let set1: HashSet<i32> = vec![1, 2, 3, 4, 5].into_iter().collect();
    let set2: HashSet<i32> = vec![4, 5, 6, 7, 8].into_iter().collect();

    // Find union, intersection, and difference
    // let union: HashSet<_> = ???
    // let intersection: HashSet<_> = ???
    // let diff: HashSet<_> = ???  // Elements in set1 but not set2
}

// Advanced: Build an index (like pandas set_index)
#[derive(Debug, Clone)]
struct Record {
    id: i32,
    name: String,
    value: f64,
}

fn build_index(records: Vec<Record>) -> HashMap<i32, Record> {
    // TODO: Create a HashMap indexed by id
    HashMap::new()
}

fn challenge_7() {
    let records = vec![
        Record { id: 1, name: "A".to_string(), value: 10.0 },
        Record { id: 2, name: "B".to_string(), value: 20.0 },
        Record { id: 3, name: "C".to_string(), value: 30.0 },
    ];

    // let index = build_index(records);
    // Now can lookup by id in O(1) time
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_group_statistics() {
        let data = vec![("A", 10.0), ("A", 20.0)];
        let stats = group_statistics(data);
        assert_eq!(stats.get("A"), Some(&(30.0, 2.0)));
    }
}
