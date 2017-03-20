use std::collections::HashSet;

mod tests;

pub fn factorial(n: i64) -> i64 {
    // (1..n + 1).product()
    match () {
        () if n == 0 || n == 1 => 1,
        _ => factorial(n -1) * n,
    }
}

pub fn rot13(cypher: &str) -> String {
    let upper: Vec<_> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
    let lower: Vec<_> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
    let mut ret = String::new();

    let _: Vec<_> = cypher.chars().map(|c| {
        let alphabeth = if c.is_lowercase() { &lower } else { &upper };

        if let Some(mut position) = alphabeth.iter().position(|&x| x == c) {
            if position + 13 > 25 { position = position - 13 } else { position += 13 }
            ret.push(alphabeth[position]);
        }

    }).collect();

    ret
}

// http://stackoverflow.com/questions/27996430/reversing-a-string-in-rust
pub fn is_palindrome(word: &str) -> bool {
    word == word.chars().rev().collect::<String>()
}

pub fn used_chars_count(chars: &[&str]) -> usize {
    let mut counter = HashSet::new();
    let chars: Vec<_> = chars.iter().flat_map(|c| {
        c.chars()
            .filter(|c| c.is_alphabetic() )
    }).collect();
    for c in chars.clone() {
        if !counter.contains(&c) { counter.insert(c); }
    };
    counter.len()
}

fn greatest_subsequencial_sum(arr: &[i64]) -> &[i64] {
    // all possible window lengths
    (1..arr.len() + 1)
        // iterate over all possible windows
        .flat_map(|len| arr.windows(len))
        // add the empty slice in case all numbers are negative
        .chain(std::iter::once(&arr[0..0]))
        // we want to select the window which sum is the greatest
        .max_by_key(|win| win.iter().sum::<i64>())
        // we know our iterator has at least one element
        .unwrap()
}
