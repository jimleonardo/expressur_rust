mod evaluator;
mod tokenizer;  
pub mod prelude{
    pub use std::collections::HashMap;
    pub use rust_decimal::Decimal;
    pub use rust_decimal_macros::*;
    pub use crate::evaluator::*;
}

pub mod expressur{
    pub use crate::evaluator::*;
}