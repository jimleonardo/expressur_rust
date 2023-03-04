use std::{collections::BTreeMap, ops::Add};

use expressur::expressur::*;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use num_format::{Locale, ToFormattedString};

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
    let expressions: BTreeMap<String, String> = [
        ("cplusaplusb".to_string(),"c + aplusb".to_string()),
        ("aplusb".to_string(),"a + b".to_string()),
        ("extraindirection".to_string(), "(aplusb/ cplusaplusb)".to_string())
    ].iter().cloned().collect();        

    let context: BTreeMap<String, Decimal> = [
        ("a".to_string(), dec!(1.)),
        ("b".to_string(), dec!(2.)),
        ("c".to_string(), dec!(4.))    
    ].iter().cloned().collect();
    // use the values stored in map
    // warm up
    evaluate_expressions(&expressions, &context).unwrap();
    let outer_loops = 10;
    let inner_loops = 100_000;
    let mut total = std::time::Duration::new(0, 0); 
    println!("Starting loop...");
    for i in 0..outer_loops {
        let duration = do_the_loop(&expressions, &context, inner_loops);
        total = total.add(duration);
        println!("Pass {}, Time to Execute {}: {:?}", i+1, inner_loops.to_formatted_string(&Locale::en), duration);
    }
    println!("Total Time to Execute {}: {:?}", (inner_loops*outer_loops).to_formatted_string(&Locale::en), total);
}

fn do_the_loop(expressions: &BTreeMap<String, String>, context: &BTreeMap<String, Decimal>, inner_loops: usize) -> std::time::Duration{
    let now = std::time::Instant::now();
    for _ in 0..inner_loops {
        evaluate_expressions(expressions, context).unwrap();
    }
    now.elapsed()    
}