// Exercise 10: Lifetimes

pub fn exercise() {
    println!("\n=== Exercise 10: Lifetimes ===");

    challenge_1();
    challenge_2();
    challenge_3();
}

fn challenge_1() {
    // TODO: Fix the lifetime issue
    // Uncomment and add lifetime annotations

    // fn first_word(s: & str) -> & str {
    //     s.split_whitespace().next().unwrap()
    // }

    // let sentence = String::from("Hello world from Rust");
    // let word = first_word(&sentence);
    // println!("First word: {}", word);
}

fn challenge_2() {
    // TODO: Implement longest with proper lifetime annotations

    // fn longest(x: &str, y: &str) -> &str {
    //     if x.len() > y.len() {
    //         x
    //     } else {
    //         y
    //     }
    // }

    // let string1 = String::from("long string");
    // let string2 = String::from("short");
    // let result = longest(&string1, &string2);
    // println!("Longest: {}", result);
}

// TODO: Create a struct that holds a reference
// struct DataSlice<???> {
//     data: &??? [i32],
//     start: usize,
//     end: usize,
// }

// TODO: Implement methods for DataSlice
// impl<???> DataSlice<???> {
//     fn new(data: &??? [i32], start: usize, end: usize) -> Self {
//         DataSlice { data, start, end }
//     }
//
//     fn get_slice(&self) -> &[i32] {
//         &self.data[self.start..self.end]
//     }
//
//     fn len(&self) -> usize {
//         self.end - self.start
//     }
// }

fn challenge_3() {
    // TODO: Use DataSlice
    // let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    // let slice = DataSlice::new(&numbers, 2, 7);
    // println!("Slice: {:?}", slice.get_slice());
    // println!("Length: {}", slice.len());
}

// Advanced: Multiple lifetimes
// fn get_first<???, ???>(x: &??? str, y: &??? str) -> &??? str {
//     // Return only x, independent of y's lifetime
//     x
// }

fn challenge_4() {
    // TODO: Test get_first with different lifetimes
}

// Advanced: Lifetime bounds
struct Context<'a> {
    data: &'a str,
}

// TODO: Implement a method that returns a reference
// The returned reference should have the same lifetime as &self
// impl<'a> Context<'a> {
//     fn get_data(&self) -> &str {
//         self.data
//     }
// }

fn challenge_5() {
    let text = String::from("Context data");
    // let ctx = Context { data: &text };
    // println!("Data: {}", ctx.get_data());
}

// Advanced: Understand lifetime constraints
fn challenge_6() {
    // This will work
    // let string1 = String::from("long string is long");
    // let result;
    // {
    //     let string2 = String::from("short");
    //     result = longest(&string1, &string2);
    //     println!("{}", result);  // Used while both are valid
    // }

    // This won't work - uncomment to see error
    // let string1 = String::from("long string");
    // let result;
    // {
    //     let string2 = String::from("short");
    //     result = longest(&string1, &string2);
    // }
    // println!("{}", result);  // ERROR: string2 dropped!
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lifetimes() {
        // Add tests for your lifetime code
    }
}
