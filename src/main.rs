mod tokenizer;  
use std::{env, collections::VecDeque};
mod prelude{
    pub use std::collections::HashMap;
    pub use std::vec;

    pub use crate::tokenizer::*;
}

use prelude::*;

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
        "(" => SUBEXPRESSION_PRECEDENCE,
        ")" => SUBEXPRESSION_PRECEDENCE,
        _ => NOT_AN_OPERATOR
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let expression = args[1].to_string();
    let context = HashMap::new();
    let result = evaluate_expression(&expression, &context);
    match result {
        Ok(value) => println!("{}", value),
        Err(error) => println!("{}", error)
    } 
}

pub fn evaluate_expression(expression: &str, context:&HashMap<String, f32>,) -> Result<f32, String> {
    let mut stack: Vec<String> = Vec::new();
    let mut q =  reverse_polish_notate(expression.to_string());
    while !q.is_empty() {
        let next = q.pop_front().unwrap();
        if operator_precedence(&next) != NOT_AN_OPERATOR {
            let y = stack.pop().unwrap();
            let x = stack.pop().unwrap();
            let x_val =  get_val(context, &x);
            let y_val =  get_val(context, &y);
            if  x_val.is_err() && y_val.is_err()  {
                return Err(format!("Unknown variables: {} and {}", x, y));
            }
            else if x_val.is_err() {
                return Err(format!("Unknown variable: {}", x));
            }
            else if y_val.is_err() {
                return Err(format!("Unknown variable: {}", y));
            }
            let result = evaluate_operator(x_val.unwrap()  ,y_val.unwrap(), &next);
            stack.push(result.to_string());
        }
        else {
            stack.push(next);
        }
    }
    Ok(stack.pop().unwrap().parse::<f32>().unwrap())    
}

pub fn evaluate_expressions(expressions: &HashMap<String, String>, context:&HashMap<String, f32>,) -> Result<HashMap<String, f32>, Vec<(String, String)>> {
    // need to change to accept a dictionary of expressions and return those.
    let mut results: HashMap<String, f32> = context.clone();
    let mut expressions_to_evaluate: Vec<(String, String)> = expressions.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect();    

    loop {
        let mut were_any_found = false;
    let mut uncalculated_expressions:HashMap<String, String> = HashMap::new();

        for expression in expressions_to_evaluate {
            let result = evaluate_expression(expression.1.as_str(), &results);
            match result{            
                Ok(value) => {   
                    results.insert(expression.0, value);
                    were_any_found = true;            
                },
                _ => {
                    uncalculated_expressions.insert(expression.0, expression.1);
                }
            }    
        }            
            expressions_to_evaluate = uncalculated_expressions.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect();
            if uncalculated_expressions.is_empty() && were_any_found {
                break;
            }
            else  if !were_any_found {
                println!("Ending loop. Could not evaluate: {}", uncalculated_expressions.keys().map(|k| k.to_string()).collect::<Vec<String>>().join(", "));
                break;                
            }
            else{
                println!("Looping back. Could not evaluate: {}", uncalculated_expressions.keys().map(|k| k.to_string()).collect::<Vec<String>>().join(", "));
            }            
    }
    if expressions_to_evaluate.is_empty(){
        Ok(results)
    }
    else {
        Err(expressions_to_evaluate)
    }

    
}

fn get_val(context: &HashMap<String, f32>, token: &String) -> Result<f32, String> {
    if token.parse::<f32>().is_ok() {
        Ok(token.parse::<f32>().unwrap())        
    }
    else if context.contains_key(token) {
        Ok(context[token])
        
    }
    else {
        Err(format!("Unknown variable: {}", token))
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
    while !operator_stack.is_empty(){ 
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
    assert_eq!(evaluate_expression(expression, &context).unwrap(), expected);
}


#[test]
fn test_eval_expr(){
    let expressions: HashMap<String, String> = [("val".to_string(),"( 1 + 2 ) * a".to_string())].iter().cloned().collect();
    let a:f32 = 3.;    
    let expected = 9.0;
    let context: HashMap<String, f32> =
    [("a".to_string(), a)]
     .iter().cloned().collect();
    // use the values stored in map
    let results = evaluate_expressions(&expressions, &context).unwrap();
    let actual = results["val"];    
    assert_eq!(actual, expected);
}

#[test]
fn test_eval_context_expr1(){
    let expressions: HashMap<String, String> = [
        ("cplusaplusb".to_string(),"c + aplusb".to_string()),
        ("aplusb".to_string(),"a + b".to_string()),
        ("extraindirection".to_string(), "(aplusb/ cplusaplusb)".to_string())
        ].iter().cloned().collect();        

    let context: HashMap<String, f32> =
    [("a".to_string(), 1.),
        ("b".to_string(), 2.),
        ("c".to_string(), 4.)    
    ].iter().cloned().collect();
    // use the values stored in map
    let results = evaluate_expressions(&expressions, &context).unwrap();

    assert_eq!(results["aplusb"], 3.);
    assert_eq!(results["cplusaplusb"], 7.);
    assert_eq!(format!("{:.3}", results["extraindirection"]), "0.429");
}