[![Build Status](https://travis-ci.org/Maaarcocr/closestmatch-rs.svg?branch=master)](https://travis-ci.org/Maaarcocr/closestmatch-rs)
# closestmatch-rs
This library fuzzy matches an input string to a list of target strings.

## Why this library
Let's say that you have a collections of words, call it a dictionary of known words, such as ["hello","good","rust"]. You also have a word, such as "hllo" and you want to find the closest match from the dictionary. This is the main functionality of this library. 

## How it works
The library is based on the concept of bags of words and it calculate the similarity splitting strings in substrings.
The developer using this library can specify how long the bags of words are.

## Documentation
You can find the documentation [here](https://maaarcocr.github.io/closestmatch-rs/)

## Example

```rust
let known_words = ["hello".to_string(), "rust".to_string(), "hello world".to_string()].to_vec();
//The second parameter specify that strings will be splitted in bags of words of length 2 and 3.
let cm = new(known_words,[2, 3].to_vec());
let closest = cm.get_closest("hlo".to_string());
println!("{}", closest)
```

This will print ```Some("hello")```.


