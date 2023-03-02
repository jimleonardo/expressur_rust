mod tokenizer;  
use phf::phf_map;
mod prelude{
    pub use std::collections::HashMap;
    pub use std::vec;
}


// const operator_precedence: HashMap<&str, i32> = [
//     ("=", 10),
//     ("^", 40),
//     ("+", 50),
//     ("-", 50),
//     ("*", 80),
//     ("/", 80),
//     ("%", 80),
//     ("(", 1000),
//     (")", 1000)
//  ].iter().cloned().collect()  ;

static operator_precedence: phf::Map<&'static str, i32> = phf_map! {
    "=" => 10,
    "^" => 40,
    "+" => 50,
    "-" => 50,
    "*" => 80,
    "/" => 80,
    "%" => 80,
    "(" => 1000,
    ")" => 1000
};
use prelude::*;

fn main() {

}

fn evaluate(expression: &str) -> f32 {
    let mut stack = Vec::new();
    let tokens =  reverse_polish_notate(expression.to_string());
    for token in tokens{
        if operator_precedence.contains_key(&token) {
            let y = stack.pop().unwrap();
            let x = stack.pop().unwrap();
            stack.push(evaluate_operator(x, y, &token));
        } else {
            stack.push(token.parse().unwrap());
        }
    }
    stack.pop().unwrap()
}


fn evaluate_operator (x: f32, y: f32, op: &str) -> f32 {
    match op {
        "=" => if x == y { 1.0 } else { 0.0 },
        "^" => x.powf(y),
        "+" => x + y,
        "-" => x - y,
        "*" => x * y,
        "/" => x / y,
        "%" => x % y,
        _ => panic!("Unknown operator: {}", op)
    }
}

fn reverse_polish_notate(expression: String) -> Vec<String>{
    // copilot results...
    let mut output = Vec::new();
    let mut stack = Vec::new();
    for token in expression.split_whitespace() {
        if token == "(" {
            stack.push(token);
        } else if token == ")" {
            while stack.last() != Some(&"(") {
                output.push(stack.pop().unwrap());
            }
            stack.pop();
        } else if operator_precedence.contains_key(token) {
            while let Some(top) = stack.last() {
                if operator_precedence[top] >= operator_precedence[token] {
                    output.push(stack.pop().unwrap());
                } else {
                    break;
                }
            }
            stack.push(token);
        } else {
            output.push(token);
        }
    }
    while let Some(top) = stack.pop() {
        output.push(top);
    }
    output.iter().map(|x| x.to_string()).collect()
}

#[test]
fn test_polish(){
    let expression = "1 + 2 * 3".to_string();
    let expected = vec!["1", "2", "3", "*", "+"];
    assert_eq!(reverse_polish_notate(expression), expected);
}

#[test]
fn test_eval(){
    let expression = "( 1. + 2. ) * 3.";
    let expected = 9.0;
    assert_eq!(evaluate(expression), expected);
}