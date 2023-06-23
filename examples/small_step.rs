use understanding_computation::semantics::small_step::*;

fn main() {
    let mut environment = Environment::new();
    environment.insert("x".to_string(), 2);
    environment.insert("y".to_string(), 1);

    let mut expression = add(
        multiply(variable("x".to_string()), number(2)),
        multiply(variable("y".to_string()), number(3))
    );

    println!("{:?}", environment);
    println!("{:?}", expression);
    loop {
        expression = match expression {
            Expression::Number { value: _ } => break,
            _ => expression.reduce(&environment)
        };
        println!("{:?}", expression);
    };

    let environment = Environment::new();
    let expression = add(number(2), number(5));
    let statement = assign("x".to_string(), expression);
    println!("{:?}", statement);
    println!("{:?}", environment);

    let (statement, environment) = statement.reduce(&environment);
    println!("{:?}", statement);
    println!("{:?}", environment);

    let (statement, environment) = statement.reduce(&environment);
    println!("{:?}", statement);
    println!("{:?}", environment);
}