use super::Environment;

#[derive(Debug, Clone, Copy)]
pub enum OperatorKind {
    Add,
    Multiply,
}

impl OperatorKind {
    fn operate(&self, left: i32, right: i32) -> i32 {
        match self {
            OperatorKind::Add => left + right,
            OperatorKind::Multiply => left * right,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Expression {
    Number {value: i32 },
    Operator {
        kind: OperatorKind,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    Variable { name: String},
}

impl Expression {

    pub fn reduce(&self, environment: &Environment) -> Expression {
        match self {
            Expression::Number { value } => number(*value),
            Expression::Operator { kind, left, right } => {
                if let Expression::Number { value: l } = **left {
                    if let Expression::Number { value: r } = **right {
                        number(kind.operate(l, r))
                    } else {
                        operator(*kind, (**left).clone(), right.reduce(environment))
                    }
                } else {
                    operator(*kind, left.reduce(environment), (**right).clone())
                }
            },
            Expression::Variable { name } => {
                let value = environment.get(name);
                match value {
                    Some(v) => number(*v),
                    None => panic!("No variable: {}", name),
                }
            }
        }
    }

}

pub fn number(value: i32) -> Expression {
    Expression::Number { value }
}

pub fn add(left: Expression, right: Expression) -> Expression {
    Expression::Operator {
        kind: OperatorKind::Add,
        left: Box::new(left),
        right: Box::new(right),
    }
}

pub fn multiply(left: Expression, right: Expression) -> Expression {
    Expression::Operator {
        kind: OperatorKind::Multiply,
        left: Box::new(left),
        right: Box::new(right),
    }
}

pub fn variable(name: String) -> Expression {
    Expression::Variable { name }
}

fn operator(kind: OperatorKind, left: Expression, right: Expression) -> Expression {
    Expression::Operator {
        kind,
        left: Box::new(left),
        right: Box::new(right),
    }
}

