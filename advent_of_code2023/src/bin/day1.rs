//Day 1: Trebuchet?!

use std::collections::HashMap;
use::std::fs;
use std::sync::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    static ref MATCH_OPTIONS_DICTIONARY: Mutex<HashMap<&'static str, i32>> = {
        let mut m = HashMap::new();
        m.insert("one", 1);
        m.insert("two", 2);
        m.insert("three", 3);
        m.insert("four", 4);
        m.insert("five", 5);
        m.insert("six", 6);
        m.insert("seven", 7);
        m.insert("eight", 8);
        m.insert("nine", 9);
        Mutex::new(m)
    };
}

fn main() {
    println!("Current working directory: {:?}", std::env::current_dir());
        let sum = fs::read_to_string(".\\src\\inputFiles\\day1.txt")
        .expect("Something went wrong reading the file")
        .lines()
        .map(|line| {
            println!("{}", line);

            let mut current_string = String::new();
            let mut first_digit: Option<i32> = None;
            let mut last_digit: Option<i32> = None;

            for c in line.chars() {
                if c.is_digit(10) {
                    first_digit = Some(c.to_digit(10).unwrap() as i32);
                    break;
                } else {
                    current_string.push(c);
                    let match_result = match_string(current_string.as_str());
                    if match_result != -1 {
                        first_digit = Some(match_result);
                        break;
                    }
                }
            }

            current_string.clear();

            for c in line.chars().rev() {
                if c.is_digit(10) {
                    last_digit = Some(c.to_digit(10).unwrap() as i32);
                    break;
                } else {
                    current_string.push(c);
                    let match_result = match_string(current_string.chars().rev().collect::<String>().as_str());
                    if match_result != -1 {
                        last_digit = Some(match_result);
                        break;
                    }
                }
            }
        
            println!("{} {}", first_digit.unwrap_or_else(|| 0), last_digit.unwrap_or_else(|| 0));
            

            let addition = first_digit.unwrap_or_else(|| 0) * 10 + last_digit.unwrap_or_else(|| 0);

            println!("{}", addition);
            addition
        })        
        .fold(0, |acc, x| {
            println!("{}", acc);
            acc + x
        });

        println!("Sum: {}", sum);
}

fn match_string(current_string: &str) -> i32 {
    let dictionary = MATCH_OPTIONS_DICTIONARY.lock().unwrap();
    for (key, &value) in dictionary.iter() {
        if current_string.contains(key) {
            return value;
        }
    }
    -1
}