mod expression;
mod statement;

use std::collections::HashMap;

pub use self::expression::*;
pub use self::statement::*;


pub type Environment = HashMap<String, i32>;