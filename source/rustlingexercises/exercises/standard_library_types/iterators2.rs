// iterators2.rs
// In this exercise, you'll learn some of the unique advantages that iterators
// can offer. Follow the steps to complete the exercise.
// As always, there are hints if you execute `rustlings hint iterators2`!



// Step 1.
// Complete the `capitalize_first` function.
// "hello" -> "Hello"
pub fn capitalize_first(input: &str) -> String {
    println!("myoutput:{}",input);
    let mut c = input.chars();
    match c.next() {
        None => String::new(),
        Some(first) => String::from(first.to_uppercase().to_string()+c.as_str()),
    }
}

// Step 2.
// Apply the `capitalize_first` function to a slice of string slices.
// Return a vector of strings.
// ["hello", "world"] -> ["Hello", "World"]
pub fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    // 方法一
    // let mut result:Vec<String> = Vec::new();
    // let content = words.iter();
    // for i in content {
    //     result.push(capitalize_first(&*i))
    // }
    // result
    //方法二 https://egghead.io/lessons/rust-rustlings-iterators3-iterating-and-collecting-values-using-the-intoiter-trait
    words.iter().map(|w| capitalize_first(w)).collect()
}

// Step 3.
// Apply the `capitalize_first` function again to a slice of string slices.
// Return a single string.
// ["hello", " ", "world"] -> "Hello World"
pub fn capitalize_words_string(words: &[&str]) -> String {
    //方法一
    // let mut result = String::new();
    // let content = words.iter();
    // for i in content {
    //     result += &capitalize_first(&*i)
    // }   
    // result
    words.iter().map(|w| capitalize_first(w)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        assert_eq!(capitalize_words_vector(&words), ["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        assert_eq!(capitalize_words_string(&words), "Hello World");
        assert!(false);
    }
}
