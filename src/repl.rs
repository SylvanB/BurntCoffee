use crate::parser::{Expression, Operator};


pub(crate) fn evaluate_expression(exp: &Expression) -> f32 {
    match exp {
        Expression::NumberLiteral(n) => *n,
        Expression::BinaryExpression(_, _, _) => evaluate_binary_expr(exp)
    }
}

pub(crate) fn evaluate_binary_expr(exp: &Expression) -> f32 {
    match exp {
        Expression::NumberLiteral(n) => *n,
        Expression::BinaryExpression(l, op, r) => {
            let right = evaluate_expression(&*r);
            let left = evaluate_expression(&*l);
            calulate_arithmetic_op(&left, op, &right)
        }
    }
}


pub(crate) fn calulate_arithmetic_op(left: &f32, op: &Operator, right: &f32) -> f32 {
    match op {
        Operator::Divide => left / right,
        Operator::Multiply => left * right,
        Operator::Add => left + right,
        Operator::Subtract => left - right,
    }
}