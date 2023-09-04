use crate::ast::{Expression, Operator};

pub fn stringify_expression(expression: Expression) -> Expression {
    Expression::Call(
        Box::new(Expression::Operation(
            Box::new(expression),
            Operator::Access,
            Box::new(Expression::Variable("to_string".to_string())),
        )),
        vec![],
    )
}
