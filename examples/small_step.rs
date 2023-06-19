use std::collections::HashMap;

use understanding_computation::semantics::small_step::*;

fn main() {
    let mut environment = HashMap::new();
    environment.insert("x", 2);
    environment.insert("y", 1);

    let mut expression = add(
        multiply(variable("x"), number(2)),
        multiply(variable("y"), number(3))
    );

    println!("{:?}", environment);
    println!("{:?}", expression);
    while !expression.is_number() {
        expression = expression.reduce(&environment);
        println!("{:?}", expression);
    }
}