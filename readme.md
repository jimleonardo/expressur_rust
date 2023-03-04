# Expressur
Expressur does some basic math. This project is a port of Expressur from C# to Rust. The original C# project is [here](https://github.com/jimleonardo/Expressur). The port is **NOT** complete or functional yet, that will take another couple of working sessions.

The real reason I built Expressur is to be a meaningful but straightforward set of code that can be ported to almost any other language so that the languages can be compared. It does this by taking a normal problem, arithmetic, and using string manipulation, iteration, and primitive operations.

It can also calculate the results of a set of formula, including formula that rely on the results of other formula. For example, this test shows this "extra indirection" where one formula relies on the results from two other formula, including a formula that in turn relies on other formula.

```rust
#[test]
fn test_eval_context_expr1(){
    let expressions: HashMap<String, String> = [
        ("cplusaplusb".to_string(),"c + aplusb".to_string()),
        ("aplusb".to_string(),"a + b".to_string()),
        ("extraindirection".to_string(), "(aplusb/ cplusaplusb)".to_string())
        ].iter().cloned().collect();        

    let context: HashMap<String, Decimal> = [
        ("a".to_string(), dec!(1.)),
            ("b".to_string(), dec!(2.)),
            ("c".to_string(), dec!(4.))    
    ].iter().cloned().collect();
    // use the values stored in map
    let results = evaluate_expressions(&expressions, &context).unwrap();

    assert_eq!(results["aplusb"], dec!(3.));
    assert_eq!(results["cplusaplusb"], dec!(7.));
    assert_eq!(results["extraindirection"].round_dp(3), dec!(0.429));
}
```

This uses the [Shunting Yard Algorithm](https://en.wikipedia.org/wiki/Shunting-yard_algorithm) to convert the expressions into [Reverse Polish Notation](https://en.wikipedia.org/wiki/Reverse_Polish_notation) in order to handle operator precedence. This is a relatively old school technique suitable for handling arithmetic expressions, but won't be a good basis for building a whole programming language.

## PseudoGrammar

*identifier* := [A-Za-z_][A-Za-z_0-9*]

*number* := ^-?[0-9]\d*(\.\d+)?$

*token* := *identifier* | *number*

*operator* := [*/+-%^=]

*expression* := [(]*expression*|*token* *operator* *expression*|*token*[)]

Expressur handles all numbers as Base-10 decimals. This will meet most end users' expectations for most scenarios.

### Operators supported

- "+" - addition (1 + 1 equals 2)
- "-" - subtraction (2 - 2 equals 0)
- "*" - multiplication (3 * 3 equals 9)
- "/" - division (4 / 4 equals 1)
- "%" - remainder (5%2 equals 1)
- "^" - power (6^6 equals 46656)
- "=" - equals (7=7 equals 1 [true], 7=9 equals 0 [false])
