// Exercise 06: Traits

pub fn exercise() {
    println!("\n=== Exercise 06: Traits ===");

    challenge_1();
    challenge_2();
    challenge_3();
}

// TODO: Define a trait called Describable
// with a method: fn describe(&self) -> String;

// TODO: Implement Describable for this struct
#[derive(Debug)]
struct Dataset {
    name: String,
    size: usize,
}

fn challenge_1() {
    // TODO: Create a Dataset and call describe() on it
    // let ds = Dataset {
    //     name: String::from("Sales Data"),
    //     size: 1000,
    // };
    // println!("{}", ds.describe());
}

// TODO: Define a trait for statistical operations
// trait Stats {
//     fn mean(&self) -> f64;
//     fn min(&self) -> f64;
//     fn max(&self) -> f64;
// }

// TODO: Implement Stats for Vec<f64>

fn challenge_2() {
    // TODO: Create a vector and use the Stats trait methods
    // let data = vec![1.5, 2.8, 3.2, 1.9, 4.1];
    // println!("Mean: {:.2}", data.mean());
    // println!("Min: {:.2}", data.min());
    // println!("Max: {:.2}", data.max());
}

// Advanced: Generic functions with trait bounds
// TODO: Uncomment when you define the Stats trait
// fn print_stats<T>(data: &T)
// where
//     T: Stats,
// {
//     // TODO: Implement this function
//     // Print mean, min, max using the trait methods
// }

fn challenge_3() {
    // TODO: Use print_stats with your Stats trait
}

// Advanced: Multiple trait bounds
trait Comparable {
    fn compare(&self, other: &Self) -> String;
}

// TODO: Implement Comparable for i32
// If self > other, return "greater"
// If self < other, return "less"
// Otherwise return "equal"

// Advanced: Trait with default implementation
trait Reportable {
    fn title(&self) -> String;

    // Default implementation
    fn report(&self) -> String {
        format!("Report: {}", self.title())
    }
}

#[derive(Debug)]
struct Experiment {
    name: String,
    trials: usize,
}

// TODO: Implement Reportable for Experiment
// You only need to implement title()

fn challenge_4() {
    // TODO: Create an Experiment and call report()
    // The default implementation should work!
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_traits() {
        // Add tests for your trait implementations
    }
}
