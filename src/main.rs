mod tokenizer;  
use std::{env, collections::VecDeque};
mod prelude{
    pub use std::collections::HashMap;
    pub use std::vec;

    pub use crate::tokenizer::*;
}

use prelude::*;

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

const NOT_AN_OPERATOR: i32 = -1;
const SUBEXPRESSION_PRECEDENCE: i32 = 1000;

fn operator_precedence(op: &str) -> i32 {    
    match op {
        "=" => 10,
        "^" => 40,
        "+" => 50,
        "-" => 50,
        "*" => 80,
        "/" => 80,
        "%" => 80,
        "(" => 1000,
        ")" => 1000,
        _ => NOT_AN_OPERATOR
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let expression = args[1].to_string();
    let context = HashMap::new();
    let result = evaluate_expression(&expression, &context);
    println!("{}", result.1);
}

fn evaluate_expression(expression: &str, context:&HashMap<String, f32>,) -> (bool, f32) {
    let mut stack: Vec<String> = Vec::new();
    let mut q =  reverse_polish_notate(expression.to_string());
    while !q.is_empty() {
        let next = q.pop_front().unwrap();
        if operator_precedence(&next) != NOT_AN_OPERATOR {
            let y = stack.pop().unwrap();
            let x = stack.pop().unwrap();
            let x_val =  get_val(context, &x);
            let y_val =  get_val(context, &y);
            if !x_val.0 || !y_val.0  {
                return (false, f32::default());
            }
            let result = evaluate_operator(x_val.1, y_val.1, &next);
            stack.push(result.to_string());
        }
        else {
            stack.push(next);
        }
    }
    (true, stack.pop().unwrap().parse::<f32>().unwrap())
}

fn evaluate_expressions(expressions: &Vec<String>, context:&HashMap<String, f32>,) -> (bool, Vec<f32>) {
    // need to change to accept a dictionary of expressions and return those.
    let mut results: Vec<f32> = Vec::new();
    for expression in expressions {
        let result = evaluate_expression(expression, context);
        if !result.0 {
            return (false, results);
        }
        results.push(result.1);
    }
    (true, results)
}

fn get_val(context: &HashMap<String, f32>, token: &String) -> (bool, f32) {
    if token.parse::<f32>().is_ok() {
        (true, token.parse::<f32>().unwrap())        
    }
    else if context.contains_key(token) {
        (true ,context[token])        
    }
    else {
        (false, f32::default())
    }
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

fn reverse_polish_notate(expression: String) -> VecDeque<String>{
    let mut output:Vec<String> = Vec::new();
    
    let mut operator_stack: Vec<(i32, String)> = Vec::new();
    let tokens = tokenize(&expression);
    for next in tokens {
        let precedence = operator_precedence(&next);
        if precedence == NOT_AN_OPERATOR {
            output.push(next);
        }
        else if precedence < SUBEXPRESSION_PRECEDENCE {
            while !operator_stack.is_empty() 
                && operator_stack.last().unwrap().0 >= precedence
                && operator_stack.last().unwrap().1 != "("
            {
                let op = operator_stack.pop().unwrap().1;
                output.push(op);
            }
            operator_stack.push((precedence, next));
        }
        else if next == "(" {
            operator_stack.push((precedence, next));
        }
        else if next == ")" {
            let mut found_left_parens = false;
            while !operator_stack.is_empty() {
                let op = operator_stack.pop().unwrap().1;
                if op != "(" {
                    output.push(op);
                }
                else {
                    found_left_parens = true;
                    break;
                }
            }
            if !found_left_parens {
                panic!("Parenthesis were not balanced in the expression {}. Missing Left Parenthesis", expression);
            }
        }
    }
    while operator_stack.len() > 0 { 
        let op = operator_stack.pop().unwrap().1;
        if op == "(" {
            panic!("Parenthesis were not balanced in the expression {}. Missing Right Parenthesis", expression);
        }
        output.push(op);
    }
            
    output.iter().map(|x| x.to_string()).collect()
}

#[test]
fn test_polish1(){
    let expression = "1 + 2 * 3".to_string();
    let expected = vec!["1", "2", "3", "*", "+"];
    assert_eq!(reverse_polish_notate(expression), expected);
}

#[test]
fn test_polish2(){
    let expression = "(1 + 2) * 3".to_string();
    let expected = vec!["1", "2", "+", "3", "*"];
    assert_eq!(reverse_polish_notate(expression), expected);
}

#[test]
fn test_eval(){
    let expression = "( 1 + 2 ) * 3";
    let expected = 9.0;
    let context = HashMap::new();
    assert_eq!(evaluate_expression(expression, &context).1, expected);
}


#[test]
fn test_eval_expr(){
    let expression = "( 1 + 2 ) * a".to_string();
    let a:f32 = 3.;    
    let expected = 9.0;
    let context: HashMap<String, f32> =
    [("a".to_string(), a)]
     .iter().cloned().collect();
    // use the values stored in map

    assert_eq!(evaluate_expressions(&vec![expression], &context).1[0], expected);
}