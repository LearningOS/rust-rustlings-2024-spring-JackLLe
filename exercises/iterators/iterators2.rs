// iterators2.rs
//
// In this exercise, you'll learn some of the unique advantages that iterators
// can offer. Follow the steps to complete the exercise.
//
// Execute `rustlings hint iterators2` or use the `hint` watch subcommand for a
// hint.

// Step 1.
// Complete the `capitalize_first` function.
// "hello" -> "Hello"
pub fn capitalize_first(input: &str) -> String {
    let mut c = input.chars();
    match c.next() {
        None => String::new(),
        Some(first) => {
          let mut str_all = first.to_uppercase().to_string();
          str_all.push_str(&input[1..]);
          str_all
        },
    }
  }
  
  // Step 2.
  // Apply the `capitalize_first` function to a slice of string slices.
  // Return a vector of strings.
  // ["hello", "world"] -> ["Hello", "World"]
  pub fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    
    let mut vec_hell:Vec<String> =Vec::new();
    let mut index =0;
    for input in words.into_iter(){
      vec_hell.insert(index, capitalize_first(input));
      index+=1;
    }
  
    vec_hell
  }
  
  // Step 3.
  // Apply the `capitalize_first` function again to a slice of string slices.
  // Return a single string.
  // ["hello", " ", "world"] -> "Hello World"
  pub fn capitalize_words_string(words: &[&str]) -> String {
   
   let mut ret = String::new();
    for input in words.into_iter(){
      ret += capitalize_first(input).as_str();
     
    }
    ret
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
    }
  }