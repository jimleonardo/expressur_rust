use rust_decimal::MathematicalOps;
use std::collections::VecDeque;

use crate::prelude::*;
use crate::tokenizer::*;

const NOT_AN_OPERATOR: i32 = -1;
const SUBEXPRESSION_PRECEDENCE: i32 = 1000;

fn operator_precedence(op: &str) -> (i32, char) {
    match op {
        "=" => (10, '='),
        "^" => (40, '^'),
        "+" => (50, '+'),
        "-" => (50, '-'),
        "*" => (80, '*'),
        "/" => (80, '/'),
        "%" => (80, '%'),
        "(" => (SUBEXPRESSION_PRECEDENCE, char::default()),
        ")" => (SUBEXPRESSION_PRECEDENCE, char::default()),
        _ => (NOT_AN_OPERATOR, char::default()),
    }
}
/// Evaluates an arithmetic expression and returns the result.
///
/// # Arguments
/// expression: The expression to evaluate
/// context: A BTreeMap of variables and their values that can be used in the expression.
///
/// # Returns
/// The result of the expression as a Decimal.
///
/// # Errors
/// If the expression contains unknown variables or is not a valid arithemetic expression, an error is returned.
///
/// # PseudoGrammar
///
/// *identifier* := [A-Za-z_][A-Za-z_0-9*]
///
/// *number* := ^-?[0-9]\d*(\.\d+)?$
///
/// *token* := *identifier* | *number*
///
/// *operator* := [*/+-%^=]
///
/// *expression* := [(]*expression*|*token* *operator* *expression*|*token*[)]
///
/// Expressur handles all numbers as Base-10 decimals. This will meet most end users' expectations for most scenarios.///
///
/// ## Operators supported
///
/// - "+" - addition (1 + 1 equals 2)
/// - "-" - subtraction (2 - 2 equals 0)
/// - "*" - multiplication (3 * 3 equals 9)
/// - "/" - division (4 / 4 equals 1)
/// - "%" - remainder (5%2 equals 1)
/// - "^" - power (6^6 equals 46656)
/// - "=" - equals (7=7 equals 1 [true], 7=9 equals 0 [false])
///
/// # Examples
///
/// ```
/// use expressur::expressur::*;
/// use rust_decimal::Decimal;
/// use rust_decimal_macros::dec;
/// let expression = "( 1 + 2 ) * 3";
/// let expected = dec!(9.0);
/// let context = std::collections::BTreeMap::new();
/// assert_eq!(evaluate_expression(expression, &context).unwrap(), expected);
/// ```

pub fn evaluate_expression(
    expression: &str,
    context: &BTreeMap<String, Decimal>,
) -> Result<Decimal, String> {
    let mut stack: Vec<String> = Vec::new();
    let mut q = reverse_polish_notate(expression.to_string());
    while !q.is_empty() {
        let next = q.pop_front().unwrap();
        let precedence = operator_precedence(&next);
        if precedence.0 != NOT_AN_OPERATOR {
            let y = stack.pop().unwrap();
            let x = stack.pop().unwrap();
            let x_val = get_val(context, &x);
            let y_val = get_val(context, &y);
            if x_val.is_none() || y_val.is_none() {
                return Err(format!("Unknown variables: {} and {}", x, y));
            } else if x_val.is_none() {
                return Err(format!("Unknown variable: {}", x));
            } else if y_val.is_none() {
                return Err(format!("Unknown variable: {}", y));
            }

            let result = evaluate_operator(x_val.unwrap(), y_val.unwrap(), precedence.1);
            stack.push(result.to_string());
        } else {
            stack.push(next);
        }
    }
    Ok(stack.pop().unwrap().parse::<Decimal>().unwrap())
}

/// Evaluates a list of arithmetic expressions and returns the results. If any expressions cannot be evaluated, they are returned in the error.
///
/// # Arguments
/// expressions: A BTreeMap of expressions to evaluate. The key is the name of the expression and the value is the expression itself. A value can be another expression.
/// context: A BTreeMap of variables and their values that can be used in the expressions.
///
/// # Returns
/// A dictionary of the results of the expressions as Decimals. This will also contain the context variables.
///  
/// # Errors
/// If any expressions contain unknown variables, an error is returned with the list of expressions that could not be evaluated.
///
/// # Examples
///
/// ```
/// use expressur::expressur::*;
/// use rust_decimal::Decimal;
/// use rust_decimal_macros::dec;
/// use std::collections::BTreeMap;
/// let expressions: BTreeMap<String, String> = [
///     ("cplusaplusb".to_string(),"c + aplusb".to_string()),
///     ("aplusb".to_string(),"a + b".to_string()),
///     ("extraindirection".to_string(), "(aplusb/ cplusaplusb)".to_string())
///     ].iter().cloned().collect();        
///
/// let context: BTreeMap<String, rust_decimal::Decimal> = [
///     ("a".to_string(), dec!(1.)),
///     ("b".to_string(), dec!(2.)),
///     ("c".to_string(), dec!(4.))    
///     ].iter().cloned().collect();
///
/// let results = evaluate_expressions(&expressions, &context).unwrap();
///
/// assert_eq!(results["aplusb"], dec!(3.));
/// assert_eq!(results["cplusaplusb"], dec!(7.));
/// assert_eq!(results["extraindirection"].round_dp(3), dec!(0.429));
/// ```
pub fn evaluate_expressions(
    expressions: &BTreeMap<String, String>,
    context: &BTreeMap<String, Decimal>,
) -> Result<BTreeMap<String, Decimal>, Vec<(String, String)>> {
    // need to change to accept a dictionary of expressions and return those.
    let mut results: BTreeMap<String, Decimal> = context.clone();
    let mut expressions_to_evaluate: Vec<(String, String)> = expressions
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect();

    loop {
        let mut were_any_found = false;
        let mut uncalculated_expressions: BTreeMap<String, String> = BTreeMap::new();

        for expression in expressions_to_evaluate {
            let result = evaluate_expression(expression.1.as_str(), &results);
            match result {
                Ok(value) => {
                    results.insert(expression.0, value);
                    were_any_found = true;
                }
                _ => {
                    uncalculated_expressions.insert(expression.0, expression.1);
                }
            }
        }
        expressions_to_evaluate = uncalculated_expressions
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();
        if uncalculated_expressions.is_empty() && were_any_found {
            break;
        } else if !were_any_found {
            if cfg!(debug_assertions) {
                println!(
                    "Ending loop. Could not evaluate: {}",
                    uncalculated_expressions
                        .keys()
                        .map(|k| k.to_string())
                        .collect::<Vec<String>>()
                        .join(", ")
                );
            }
            break;
        } else if cfg!(debug_assertions) {
            println!(
                "Looping back. Could not evaluate: {}",
                uncalculated_expressions
                    .keys()
                    .map(|k| k.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            );
        }
    }
    if expressions_to_evaluate.is_empty() {
        Ok(results)
    } else {
        Err(expressions_to_evaluate)
    }
}

fn get_val(context: &BTreeMap<String, Decimal>, token: &String) -> Option<Decimal> {
    match Decimal::from_str_exact(token) {
        Ok(value) => Some(value),
        _ => {
            if context.contains_key(token) {
                Some(context[token])
            } else {
                None
            }
        }
    }
}

fn evaluate_operator(x: Decimal, y: Decimal, op: char) -> Decimal {
    match op {
        '=' => {
            if x == y {
                dec!(1.0)
            } else {
                dec!(0.0)
            }
        }
        '^' => x.powd(y),
        '+' => x + y,
        '-' => x - y,
        '*' => x * y,
        '/' => x / y,
        '%' => x % y,
        _ => panic!("Unknown operator: {}", op),
    }
}

fn reverse_polish_notate(expression: String) -> VecDeque<String> {
    let mut output: Vec<String> = Vec::new();

    let mut operator_stack: Vec<(i32, String)> = Vec::new();
    let tokens = tokenize(&expression);
    for next in tokens {
        let precedence = operator_precedence(&next);
        if precedence.0 == NOT_AN_OPERATOR {
            output.push(next);
        } else if precedence.0 < SUBEXPRESSION_PRECEDENCE {
            while !operator_stack.is_empty()
                && operator_stack.last().unwrap().0 >= precedence.0
                && operator_stack.last().unwrap().1 != "("
            {
                let op = operator_stack.pop().unwrap().1;
                output.push(op);
            }
            operator_stack.push((precedence.0, next));
        } else if next == "(" {
            operator_stack.push((precedence.0, next));
        } else if next == ")" {
            let mut found_left_parens = false;
            while !operator_stack.is_empty() {
                let op = operator_stack.pop().unwrap().1;
                if op != "(" {
                    output.push(op);
                } else {
                    found_left_parens = true;
                    break;
                }
            }
            if !found_left_parens {
                panic!(
                    "Parenthesis were not balanced in the expression {}. Missing Left Parenthesis",
                    expression
                );
            }
        }
    }
    while !operator_stack.is_empty() {
        let op = operator_stack.pop().unwrap().1;
        if op == "(" {
            panic!(
                "Parenthesis were not balanced in the expression {}. Missing Right Parenthesis",
                expression
            );
        }
        output.push(op);
    }

    output.iter().map(|x| x.to_string()).collect()
}

#[test]
fn test_reverse_polish_notate_1() {
    let expression = "1 + 2 * 3".to_string();
    let expected = vec!["1", "2", "3", "*", "+"];
    assert_eq!(reverse_polish_notate(expression), expected);
}

#[test]
fn test_reverse_polish_notate_2() {
    let expression = "(1 + 2) * 3".to_string();
    let expected = vec!["1", "2", "+", "3", "*"];
    assert_eq!(reverse_polish_notate(expression), expected);
}

#[test]
fn test_evaluate_expression() {
    let expression = "( 1 + 2 ) * 3";
    let expected = dec!(9.0);
    let context = BTreeMap::new();
    assert_eq!(evaluate_expression(expression, &context).unwrap(), expected);
}

#[test]
fn test_evaluate_expressions() {
    let expressions: BTreeMap<String, String> = [("val".to_string(), "( 1 + 2 ) * a".to_string())]
        .iter()
        .cloned()
        .collect();
    let a: Decimal = dec!(3.);
    let expected = dec!(9.0);
    let context: BTreeMap<String, Decimal> = [("a".to_string(), a)].iter().cloned().collect();
    // use the values stored in map
    let results = evaluate_expressions(&expressions, &context).unwrap();
    let actual = results["val"];
    assert_eq!(actual, expected);
}

#[test]
fn test_context_evaluate_expressions() {
    let expressions: BTreeMap<String, String> = [
        ("cplusaplusb".to_string(), "c + aplusb".to_string()),
        ("aplusb".to_string(), "a + b".to_string()),
        (
            "extraindirection".to_string(),
            "(aplusb/ cplusaplusb)".to_string(),
        ),
    ]
    .iter()
    .cloned()
    .collect();

    let context: BTreeMap<String, Decimal> = [
        ("a".to_string(), dec!(1.)),
        ("b".to_string(), dec!(2.)),
        ("c".to_string(), dec!(4.)),
    ]
    .iter()
    .cloned()
    .collect();
    // use the values stored in map
    let results = evaluate_expressions(&expressions, &context).unwrap();

    assert_eq!(results["aplusb"], dec!(3.));
    assert_eq!(results["cplusaplusb"], dec!(7.));
    assert_eq!(results["extraindirection"].round_dp(3), dec!(0.429));
}

#[test]
fn test_batch_evaluate_expressions() {
    let tests = vec![
        ("1-1", dec!(0.)),
        ("1+1", dec!(2.)),
        ("1 + 1", dec!(2.)),
        ("1 + -1", dec!(0.)),
        ("1 - 1", dec!(0.)),
        ("1 - -1", dec!(2.)),
        ("-1 - -1", dec!(0.)),
        ("-1 - +1", dec!(-2.)),
        ("-1-+1", dec!(-2.)),
        ("-14-+12/(-2*-54)", dec!(-14.111)),
        ("1 + 1", dec!(2.)),
        ("1 + (-1 + 2)", dec!(2.)),
        ("1 + 1.0", dec!(2)),
        ("1 + .0", dec!(1.)),
        ("2 ^ 4", dec!(16.)),
        ("1 + 2.2", dec!(3.2)),
        ("(1 + 1)*2", dec!(4.)),
        ("2 / 4", dec!(0.5)),
        ("1 +555", dec!(556.)),
        ("1+ 555", dec![556.]),
    ];
    for test in tests {
        let context = BTreeMap::new();
        assert_eq!(
            evaluate_expression(test.0, &context).unwrap().round_dp(3),
            test.1,
            "Failed to evaluate: {}",
            test.0
        );
    }
}
