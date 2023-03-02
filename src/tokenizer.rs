use crate::prelude::*;

pub fn tokenize(expression: &str) -> Vec<String>{

    let mut output: Vec<String> = Vec::new();
    let mut last_char = char::default();

    let mut current_token: String = String::new();
    
    for i in (0..expression.len()){
        let c = expression.chars().nth(i).unwrap();       
        
        if is_whitespace(c){
            current_token = output_token(&mut output, &mut current_token);
        }
        else if is_token_character(c){
            current_token.push(c);
        }
        else if c == '(' || c == ')' || c =='*' || c =='/' || c =='^' || c =='%' || c =='='{
            current_token = output_token(&mut output, &mut current_token);
            output.push(c.to_string());
        }
        else if c == '-' || c == '+' {
            let next = expression.chars().nth(i+1).unwrap();
            if last_char == char::default() {
                current_token.push(c);
            }
            else if is_whitespace(c) || is_operator(next) || next =='('{
                current_token = output_token(&mut output, &mut current_token);
                output.push(c.to_string());
            }
            else if is_number(next) && (is_operator(last_char) || is_whitespace(last_char) || last_char == '('){
                current_token.push(c);
            }
            else{
                current_token = output_token(&mut output, &mut current_token);
                output.push(c.to_string());
            }
            
         }
        else{
            panic!("Unexpected character: {}", c);
        }
        last_char = c;

    }
    output_token(&mut output, &mut current_token);
    output

}

fn output_token(output: &mut Vec<String>, current_token: &mut String) -> String {
    if current_token.len() > 0 {
        output.push(current_token.clone());
        current_token.clear();
    }
    current_token.clone()
}

fn is_number(c: char) -> bool {
    c.is_digit(10) || c == '.'
}

fn is_whitespace(c: char) -> bool {
    c.is_whitespace()
}

fn is_token_character(c: char) -> bool {
    c.is_alphanumeric() ||  c == '_' || is_number(c)
}

fn is_operator(c: char) -> bool {
    c == '+' || c == '-' || c == '*' || c == '/' || c == '^' || c == '=' || c == '%' 
}

#[test]
fn test_tokenizer() {
    let tests = vec![
        ("1-1", vec!["1", "-", "1"]),
        ("1+1", vec!["1", "+", "1"]),
        ("1 + 1", vec!["1", "+", "1"]),
        ("1 + -1", vec!["1", "+", "-1"]),
        ("1 - 1", vec!["1", "-", "1"]),
        ("1 - -1", vec!["1", "-", "-1"]),
        ("-1 - -1", vec!["-1", "-", "-1"]),
        ("-1 - +1", vec!["-1", "-", "+1"]),
        ("-1-+1", vec!["-1", "-", "+1"]),
        ("-14-+12/(-2*-54)", vec!["-14", "-", "+12", "/", "(", "-2", "*", "-54", ")"]),
        ("1 + 1", vec!["1", "+", "1"]),
        ("1 + (-1 + 2)", vec!["1", "+", "(", "-1", "+", "2", ")"]),
        ("1 * a", vec!["1", "*", "a"]),
        ("1 + 1.0", vec!["1", "+", "1.0"]),
        ("1 + .0", vec!["1", "+", ".0"]),
        ("1 + abn", vec!["1", "+", "abn"]),
        ("1 ^ abn", vec!["1", "^", "abn"]),
        ("1 + abn.b", vec!["1", "+", "abn.b"]),
        ("(1 + 1)*2", vec!["(", "1", "+", "1", ")", "*", "2"]),
        ("(1 + cash.cycle)*2", vec!["(", "1", "+", "cash.cycle", ")", "*", "2"]),
        ("2 / 1", vec!["2", "/", "1"]),
    ];
    for test in tests {
        assert_eq!(tokenize(test.0), test.1, "Failed to tokenize: {}", test.0);
    }
}