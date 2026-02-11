// Exercise 07: Generics

pub fn exercise() {
    println!("\n=== Exercise 07: Generics ===");

    challenge_1();
    challenge_2();
    challenge_3();
    challenge_4();
}

fn challenge_1() {
    // TODO: Create a generic function that returns the first element
    // fn first<T>(slice: &[T]) -> Option<&T> { ??? }

    let numbers = vec![1, 2, 3];
    let words = vec!["hello", "world"];

    // match first(&numbers) {
    //     Some(n) => println!("First number: {}", n),
    //     None => println!("Empty"),
    // }
}

fn first<T>(slice: &[T]) -> Option<&T> {
    // TODO: Implement this
    None
}

// TODO: Create a generic struct to hold a pair of values
// struct Pair<T, U> {
//     first: T,
//     second: U,
// }

fn challenge_2() {
    // TODO: Create pairs of different types
    // let pair1 = Pair { first: 1, second: "one" };
    // let pair2 = Pair { first: 3.14, second: true };
}

// TODO: Implement methods for Pair
// impl<T, U> Pair<T, U> {
//     fn new(first: T, second: U) -> Self { ??? }
//     fn first(&self) -> &T { ??? }
//     fn second(&self) -> &U { ??? }
// }

fn challenge_3() {
    // TODO: Use the Pair methods
}

// Advanced: Generic with trait bounds
fn find_max<T>(slice: &[T]) -> Option<&T>
where
    T: PartialOrd,
{
    // TODO: Implement this
    // Find and return the maximum element
    // Hint: Use .iter() and .max_by()
    None
}

fn challenge_4() {
    let numbers = vec![3, 7, 2, 9, 1];
    // match find_max(&numbers) {
    //     Some(max) => println!("Max: {}", max),
    //     None => println!("Empty"),
    // }
}

// Advanced: Generic data container
struct Container<T> {
    items: Vec<T>,
}

// TODO: Implement methods for Container
// impl<T> Container<T> {
//     fn new() -> Self { ??? }
//     fn add(&mut self, item: T) { ??? }
//     fn get(&self, index: usize) -> Option<&T> { ??? }
//     fn len(&self) -> usize { ??? }
// }

// TODO: Add a method that only works when T implements Display
// impl<T: std::fmt::Display> Container<T> {
//     fn print_all(&self) { ??? }
// }

fn challenge_5() {
    // TODO: Create containers of different types
    // let mut ints = Container::new();
    // ints.add(1);
    // ints.add(2);
    // ints.add(3);

    // let mut strings = Container::new();
    // strings.add(String::from("hello"));
    // strings.add(String::from("world"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first() {
        assert_eq!(first(&[1, 2, 3]), Some(&1));
        assert_eq!(first(&[] as &[i32]), None);
    }

    #[test]
    fn test_find_max() {
        assert_eq!(find_max(&[1, 5, 3, 2]), Some(&5));
        assert_eq!(find_max(&[] as &[i32]), None);
    }
}
