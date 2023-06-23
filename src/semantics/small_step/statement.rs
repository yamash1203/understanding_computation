use super::{expression::Expression, Environment};

#[derive(Debug)]
pub enum Statement {
    DoNothing,
    Assign {
        name: String,
        expression: Expression
    },
}

impl Statement {

    pub fn reduce(&self, environment: &Environment) -> (Statement, Environment) {
        match self {
            Statement::DoNothing => (do_nothing(), environment.clone()),
            Statement::Assign { name, expression } => {
                match expression {
                    Expression::Number { value } => {
                        let mut new_environment = environment.clone();
                        new_environment.insert(name.to_string(), *value);
                        (do_nothing(), new_environment)
                    },
                    _ => {
                        let assign = assign(name.to_string(), expression.reduce(environment));
                        (assign, environment.clone())
                    }
                }
            },
        }
    }

}

pub fn do_nothing() -> Statement {
    Statement::DoNothing
}

pub fn assign(name: String, expression: Expression) -> Statement {
    Statement::Assign { name, expression }
}
