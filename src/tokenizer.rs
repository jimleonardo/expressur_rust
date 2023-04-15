use crate::output_token;

pub fn tokenize(expression: &str) -> Vec<String>{

    let mut output: Vec<String> = Vec::new();
    let mut last_char = char::default();

    let mut current_token: String = String::new();
    let expression_len = expression.chars().count();
    for i in 0..expression_len{
        let c = expression.chars().nth(i).unwrap();

        if is_whitespace(c){
            current_token = output_token!(output, current_token);
        }
        else if is_token_character(c){
            current_token.push(c);
        }
        else if c == '(' || c == ')' || c =='*' || c =='/' || c =='^' || c =='%' || c =='='{
            current_token = output_token!(output, current_token);
            output.push(c.to_string());
        }
        else if c == '-' || c == '+' {
                // could be indicating negative/positive number or operator
                // if it is the first valid token in an expression/subexpression, its a negative/positive number
                // if it is the first valid token after an operator, its a negative/positive number
                // if it is the first valid token after an open parenthesis, its a negative/positive number
                // if it is the first valid token after a decimal point, its a negative/positive number
                // if it is the first valid token after an identifier or number, its an operator        
            
            let last_token = match output.last() {
                Some(token) => token.to_string(),
                None => "".to_string(),
            };
            if last_char == char::default() {
                current_token.push(c);
            }            
            else if expression_len > i {
                let next = expression.chars().nth(i+1).unwrap();

                if (is_whitespace(c) || is_operator(next) || next =='(')
                    ||
                    (is_number(next)  && (last_token != "(" || output.is_empty()) && !is_operator_str(last_token)) 
                {
                    current_token = output_token!(output, current_token);
                    output.push(c.to_string());
                }
                else if is_number(next) && (is_operator(last_char) || is_whitespace(last_char) || last_char == '('){
                    current_token.push(c);
                }
                else {
                    current_token = output_token!(output, current_token);
                    output.push(c.to_string());
                }
            }
         }
        else{
            panic!("Unexpected character: {}", c);
        }        
        last_char = c;
    }

    output_token!(output, current_token);

    output

}

#[macro_export]
macro_rules! output_token {
    ($output:ident, $current_token:ident) => {{
        if !$current_token.is_empty() {
            $output.push($current_token);
        }        
        String::default()
    }        
    };
}


fn is_number(c: char) -> bool {
    c.is_ascii_digit() || c == '.'
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

fn is_operator_str(s: String) -> bool {
    matches!(s.as_str(), "+" | "-" | "*" | "/" | "^" | "=" | "%")
}

#[test]
fn test_batch_tokenize() {
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
        ("1 +555", vec!["1", "+", "555"]),
        ("1+ 555", vec!["1", "+", "555"]),
    ];
    for test in tests {
        assert_eq!(tokenize(test.0), test.1, "Failed to tokenize: {}", test.0);
    }
}