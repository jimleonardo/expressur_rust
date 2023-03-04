use std::env;
use std::collections::BTreeMap;
use expressur::expressur::*;


/// Evaluates an arithmetic expression passed on the command line.
/// 
/// # Arguments
/// Any string that's a value or an arithmetic expression.
/// 
/// # Examples
/// 
/// ```
/// $ cargo run -- "1 + 2"
/// 3
/// ```
fn main() {
    let args: Vec<String> = env::args().collect();
    let expression = args[1].to_string();
    let context = BTreeMap::new();
    let result = evaluate_expression(&expression, &context);
    match result {
        Ok(value) => println!("{}", value),
        Err(error) => println!("{}", error)
    } 
}
