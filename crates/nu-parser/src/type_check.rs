use nu_protocol::{
    ast::{Bits, Boolean, Comparison, Expr, Expression, Math, Operator},
    engine::StateWorkingSet,
    ParseError, Type,
};

pub fn type_compatible(lhs: &Type, rhs: &Type) -> bool {
    match (lhs, rhs) {
        (Type::List(c), Type::List(d)) => type_compatible(c, d),
        (Type::Number, Type::Int) => true,
        (Type::Number, Type::Float) => true,
        (Type::Closure, Type::Block) => true,
        (Type::Any, _) => true,
        (_, Type::Any) => true,
        (Type::Record(fields_lhs), Type::Record(fields_rhs)) => {
            // Structural subtyping
            'outer: for field_lhs in fields_lhs {
                for field_rhs in fields_rhs {
                    if field_lhs.0 == field_rhs.0 {
                        if type_compatible(&field_lhs.1, &field_rhs.1) {
                            continue 'outer;
                        } else {
                            return false;
                        }
                    }
                }
                return false;
            }
            true
        }
        (lhs, rhs) => lhs == rhs,
    }
}

pub fn math_result_type(
    _working_set: &StateWorkingSet,
    lhs: &mut Expression,
    op: &mut Expression,
    rhs: &mut Expression,
) -> (Type, Option<ParseError>) {
    match &op.expr {
        Expr::Operator(operator) => match operator {
            Operator::Math(Math::Plus) => match (&lhs.ty, &rhs.ty) {
                (Type::Int, Type::Int) => (Type::Int, None),
                (Type::Float, Type::Int) => (Type::Float, None),
                (Type::Int, Type::Float) => (Type::Float, None),
                (Type::Float, Type::Float) => (Type::Float, None),
                (Type::String, Type::String) => (Type::String, None),
                (Type::Date, Type::Duration) => (Type::Date, None),
                (Type::Duration, Type::Duration) => (Type::Duration, None),
                (Type::Filesize, Type::Filesize) => (Type::Filesize, None),

                (Type::Custom(a), Type::Custom(b)) if a == b => (Type::Custom(a.to_string()), None),
                (Type::Custom(a), _) => (Type::Custom(a.to_string()), None),

                (Type::Any, _) => (Type::Any, None),
                (_, Type::Any) => (Type::Any, None),
                (
                    Type::Int
                    | Type::Float
                    | Type::String
                    | Type::Date
                    | Type::Duration
                    | Type::Filesize,
                    _,
                ) => {
                    *op = Expression::garbage(op.span);
                    (
                        Type::Any,
                        Some(ParseError::UnsupportedOperationRHS(
                            "addition".into(),
                            op.span,
                            lhs.span,
                            lhs.ty.clone(),
                            rhs.span,
                            rhs.ty.clone(),
                        )),
                    )
                }
                _ => {
                    *op = Expression::garbage(op.span);
                    (
                        Type::Any,
                        Some(ParseError::UnsupportedOperationLHS(
                            "addition".into(),
                            op.span,
                            lhs.span,
                            lhs.ty.clone(),
                        )),
                    )
                }
            },
            Operator::Math(Math::Append) => match (&lhs.ty, &rhs.ty) {
                (Type::List(a), Type::List(b)) => {
                    if a == b {
                        (Type::List(a.clone()), None)
                    } else {
                        (Type::List(Box::new(Type::Any)), None)
                    }
                }
                (Type::List(a), b) | (b, Type::List(a)) => {
                    if a == &Box::new(b.clone()) {
                        (Type::List(a.clone()), None)
                    } else {
                        (Type::List(Box::new(Type::Any)), None)
                    }
                }
                (Type::Table(a), Type::Table(_)) => (Type::Table(a.clone()), None),
                (Type::String, Type::String) => (Type::String, None),
                (Type::Binary, Type::Binary) => (Type::Binary, None),
                (Type::Any, _) | (_, Type::Any) => (Type::Any, None),
                (Type::Table(_) | Type::String | Type::Binary, _) => {
                    *op = Expression::garbage(op.span);
                    (
                        Type::Any,
                        Some(ParseError::UnsupportedOperationRHS(
                            "append".into(),
                            op.span,
                            lhs.span,
                            lhs.ty.clone(),
                            rhs.span,
                            rhs.ty.clone(),
                        )),
                    )
                }
                _ => {
                    *op = Expression::garbage(op.span);
                    (
                        Type::Any,
                        Some(ParseError::UnsupportedOperationLHS(
                            "append".into(),
                            op.span,
                            lhs.span,
                            lhs.ty.clone(),
                        )),
                    )
                }
            },
            Operator::Math(Math::Minus) => match (&lhs.ty, &rhs.ty) {
                (Type::Int, Type::Int) => (Type::Int, None),
                (Type::Float, Type::Int) => (Type::Float, None),
                (Type::Int, Type::Float) => (Type::Float, None),
                (Type::Float, Type::Float) => (Type::Float, None),
                (Type::Date, Type::Date) => (Type::Duration, None),
                (Type::Date, Type::Duration) => (Type::Date, None),
                (Type::Duration, Type::Duration) => (Type::Duration, None),
                (Type::Filesize, Type::Filesize) => (Type::Filesize, None),

                (Type::Custom(a), Type::Custom(b)) if a == b => (Type::Custom(a.to_string()), None),
                (Type::Custom(a), _) => (Type::Custom(a.to_string()), None),

                (Type::Any, _) => (Type::Any, None),
                (_, Type::Any) => (Type::Any, None),
                (Type::Int | Type::Float | Type::Date | Type::Duration | Type::Filesize, _) => {
                    *op = Expression::garbage(op.span);
                    (
                        Type::Any,
                        Some(ParseError::UnsupportedOperationRHS(
                            "subtraction".into(),
                            op.span,
                            lhs.span,
                            lhs.ty.clone(),
                            rhs.span,
                            rhs.ty.clone(),
                        )),
                    )
                }
                _ => {
                    *op = Expression::garbage(op.span);
                    (
                        Type::Any,
                        Some(ParseError::UnsupportedOperationLHS(
                            "subtraction".into(),
                            op.span,
                            lhs.span,
                            lhs.ty.clone(),
                        )),
                    )
                }
            },
            Operator::Math(Math::Multiply) => match (&lhs.ty, &rhs.ty) {
                (Type::Int, Type::Int) => (Type::Int, None),
                (Type::Float, Type::Int) => (Type::Float, None),
                (Type::Int, Type::Float) => (Type::Float, None),
                (Type::Float, Type::Float) => (Type::Float, None),
                (Type::Filesize, Type::Int) => (Type::Filesize, None),
                (Type::Int, Type::Filesize) => (Type::Filesize, None),
                (Type::Filesize, Type::Float) => (Type::Filesize, None),
                (Type::Float, Type::Filesize) => (Type::Filesize, None),
                (Type::Duration, Type::Int) => (Type::Duration, None),
                (Type::Int, Type::Duration) => (Type::Duration, None),
                (Type::Duration, Type::Float) => (Type::Duration, None),
                (Type::Float, Type::Duration) => (Type::Duration, None),
                (Type::Int, Type::String) => (Type::String, None),
                (Type::String, Type::Int) => (Type::String, None),
                (Type::Int, Type::List(a)) => (Type::List(a.clone()), None),
                (Type::List(a), Type::Int) => (Type::List(a.clone()), None),

                (Type::Custom(a), Type::Custom(b)) if a == b => (Type::Custom(a.to_string()), None),
                (Type::Custom(a), _) => (Type::Custom(a.to_string()), None),

                (Type::Any, _) => (Type::Any, None),
                (_, Type::Any) => (Type::Any, None),
                (Type::Int, _)
                | (Type::Float, _)
                | (Type::String, _)
                | (Type::Date, _)
                | (Type::Duration, _)
                | (Type::Filesize, _)
                | (Type::List(_), _) => {
                    *op = Expression::garbage(op.span);
                    (
                        Type::Any,
                        Some(ParseError::UnsupportedOperationRHS(
                            "multiplication".into(),
                            op.span,
                            lhs.span,
                            lhs.ty.clone(),
                            rhs.span,
                            rhs.ty.clone(),
                        )),
                    )
                }
                _ => {
                    *op = Expression::garbage(op.span);
                    (
                        Type::Any,
                        Some(ParseError::UnsupportedOperationLHS(
                            "multiplication".into(),
                            op.span,
                            lhs.span,
                            lhs.ty.clone(),
                        )),
                    )
                }
            },
            Operator::Math(Math::Pow) => match (&lhs.ty, &rhs.ty) {
                (Type::Int, Type::Int) => (Type::Int, None),
                (Type::Float, Type::Int) => (Type::Float, None),
                (Type::Int, Type::Float) => (Type::Float, None),
                (Type::Float, Type::Float) => (Type::Float, None),

                (Type::Custom(a), Type::Custom(b)) if a == b => (Type::Custom(a.to_string()), None),
                (Type::Custom(a), _) => (Type::Custom(a.to_string()), None),

                (Type::Any, _) => (Type::Any, None),
                (_, Type::Any) => (Type::Any, None),
                (Type::Int | Type::Float, _) => {
                    *op = Expression::garbage(op.span);
                    (
                        Type::Any,
                        Some(ParseError::UnsupportedOperationRHS(
                            "exponentiation".into(),
                            op.span,
                            lhs.span,
                            lhs.ty.clone(),
                            rhs.span,
                            rhs.ty.clone(),
                        )),
                    )
                }
                _ => {
                    *op = Expression::garbage(op.span);
                    (
                        Type::Any,
                        Some(ParseError::UnsupportedOperationLHS(
                            "exponentiation".into(),
                            op.span,
                            lhs.span,
                            lhs.ty.clone(),
                        )),
                    )
                }
            },
            Operator::Math(Math::Divide) | Operator::Math(Math::Modulo) => match (&lhs.ty, &rhs.ty)
            {
                (Type::Int, Type::Int) => (Type::Int, None),
                (Type::Float, Type::Int) => (Type::Float, None),
                (Type::Int, Type::Float) => (Type::Float, None),
                (Type::Float, Type::Float) => (Type::Float, None),
                (Type::Filesize, Type::Filesize) => (Type::Float, None),
                (Type::Filesize, Type::Int) => (Type::Filesize, None),
                (Type::Filesize, Type::Float) => (Type::Filesize, None),
                (Type::Duration, Type::Duration) => (Type::Float, None),
                (Type::Duration, Type::Int) => (Type::Duration, None),
                (Type::Duration, Type::Float) => (Type::Duration, None),

                (Type::Custom(a), Type::Custom(b)) if a == b => (Type::Custom(a.to_string()), None),
                (Type::Custom(a), _) => (Type::Custom(a.to_string()), None),

                (Type::Any, _) => (Type::Any, None),
                (_, Type::Any) => (Type::Any, None),
                (Type::Int | Type::Float | Type::Filesize | Type::Duration, _) => {
                    *op = Expression::garbage(op.span);
                    (
                        Type::Any,
                        Some(ParseError::UnsupportedOperationRHS(
                            "division".into(),
                            op.span,
                            lhs.span,
                            lhs.ty.clone(),
                            rhs.span,
                            rhs.ty.clone(),
                        )),
                    )
                }
                _ => {
                    *op = Expression::garbage(op.span);
                    (
                        Type::Any,
                        Some(ParseError::UnsupportedOperationLHS(
                            "division".into(),
                            op.span,
                            lhs.span,
                            lhs.ty.clone(),
                        )),
                    )
                }
            },
            Operator::Math(Math::FloorDivision) => match (&lhs.ty, &rhs.ty) {
                (Type::Int, Type::Int) => (Type::Int, None),
                (Type::Float, Type::Int) => (Type::Int, None),
                (Type::Int, Type::Float) => (Type::Int, None),
                (Type::Float, Type::Float) => (Type::Int, None),
                (Type::Filesize, Type::Filesize) => (Type::Int, None),
                (Type::Filesize, Type::Int) => (Type::Filesize, None),
                (Type::Filesize, Type::Float) => (Type::Filesize, None),
                (Type::Duration, Type::Duration) => (Type::Int, None),
                (Type::Duration, Type::Int) => (Type::Duration, None),
                (Type::Duration, Type::Float) => (Type::Duration, None),

                (Type::Any, _) => (Type::Any, None),
                (_, Type::Any) => (Type::Any, None),
                (Type::Int | Type::Float | Type::Filesize | Type::Duration, _) => {
                    *op = Expression::garbage(op.span);
                    (
                        Type::Any,
                        Some(ParseError::UnsupportedOperationRHS(
                            "floor division".into(),
                            op.span,
                            lhs.span,
                            lhs.ty.clone(),
                            rhs.span,
                            rhs.ty.clone(),
                        )),
                    )
                }
                _ => {
                    *op = Expression::garbage(op.span);
                    (
                        Type::Any,
                        Some(ParseError::UnsupportedOperationLHS(
                            "floor division".into(),
                            op.span,
                            lhs.span,
                            lhs.ty.clone(),
                        )),
                    )
                }
            },
            Operator::Boolean(Boolean::And)
            | Operator::Boolean(Boolean::Or)
            | Operator::Boolean(Boolean::Xor) => {
                match (&lhs.ty, &rhs.ty) {
                    (Type::Bool, Type::Bool) => (Type::Bool, None),

                    (Type::Custom(a), Type::Custom(b)) if a == b => {
                        (Type::Custom(a.to_string()), None)
                    }
                    (Type::Custom(a), _) => (Type::Custom(a.to_string()), None),

                    (Type::Any, _) => (Type::Any, None),
                    (_, Type::Any) => (Type::Any, None),

                    // FIX ME. This is added because there is no type output for custom function
                    // definitions. As soon as that syntax is added this should be removed
                    (a, b) if a == b => (Type::Bool, None),
                    (Type::Bool, _) => {
                        *op = Expression::garbage(op.span);
                        (
                            Type::Any,
                            Some(ParseError::UnsupportedOperationRHS(
                                "boolean operation".into(),
                                op.span,
                                lhs.span,
                                lhs.ty.clone(),
                                rhs.span,
                                rhs.ty.clone(),
                            )),
                        )
                    }
                    _ => {
                        *op = Expression::garbage(op.span);
                        (
                            Type::Any,
                            Some(ParseError::UnsupportedOperationLHS(
                                "boolean operation".into(),
                                op.span,
                                lhs.span,
                                lhs.ty.clone(),
                            )),
                        )
                    }
                }
            }
            Operator::Comparison(Comparison::LessThan) => match (&lhs.ty, &rhs.ty) {
                (Type::Int, Type::Int) => (Type::Bool, None),
                (Type::Float, Type::Int) => (Type::Bool, None),
                (Type::Int, Type::Float) => (Type::Bool, None),
                (Type::Float, Type::Float) => (Type::Bool, None),
                (Type::Duration, Type::Duration) => (Type::Bool, None),
                (Type::Filesize, Type::Filesize) => (Type::Bool, None),

                (Type::Custom(a), Type::Custom(b)) if a == b => (Type::Custom(a.to_string()), None),
                (Type::Custom(a), _) => (Type::Custom(a.to_string()), None),

                (Type::Nothing, _) => (Type::Nothing, None),
                (_, Type::Nothing) => (Type::Nothing, None),

                (Type::Any, _) => (Type::Bool, None),
                (_, Type::Any) => (Type::Bool, None),
                (Type::Int | Type::Float | Type::Duration | Type::Filesize, _) => {
                    *op = Expression::garbage(op.span);
                    (
                        Type::Any,
                        Some(ParseError::UnsupportedOperationRHS(
                            "less-than comparison".into(),
                            op.span,
                            lhs.span,
                            lhs.ty.clone(),
                            rhs.span,
                            rhs.ty.clone(),
                        )),
                    )
                }
                _ => {
                    *op = Expression::garbage(op.span);
                    (
                        Type::Any,
                        Some(ParseError::UnsupportedOperationLHS(
                            "less-than comparison".into(),
                            op.span,
                            lhs.span,
                            lhs.ty.clone(),
                        )),
                    )
                }
            },
            Operator::Comparison(Comparison::LessThanOrEqual) => match (&lhs.ty, &rhs.ty) {
                (Type::Int, Type::Int) => (Type::Bool, None),
                (Type::Float, Type::Int) => (Type::Bool, None),
                (Type::Int, Type::Float) => (Type::Bool, None),
                (Type::Float, Type::Float) => (Type::Bool, None),
                (Type::Duration, Type::Duration) => (Type::Bool, None),
                (Type::Filesize, Type::Filesize) => (Type::Bool, None),

                (Type::Custom(a), Type::Custom(b)) if a == b => (Type::Custom(a.to_string()), None),
                (Type::Custom(a), _) => (Type::Custom(a.to_string()), None),

                (Type::Nothing, _) => (Type::Nothing, None),
                (_, Type::Nothing) => (Type::Nothing, None),

                (Type::Any, _) => (Type::Bool, None),
                (_, Type::Any) => (Type::Bool, None),
                (Type::Int | Type::Float | Type::Duration | Type::Filesize, _) => {
                    *op = Expression::garbage(op.span);
                    (
                        Type::Any,
                        Some(ParseError::UnsupportedOperationRHS(
                            "less-than or equal comparison".into(),
                            op.span,
                            lhs.span,
                            lhs.ty.clone(),
                            rhs.span,
                            rhs.ty.clone(),
                        )),
                    )
                }
                _ => {
                    *op = Expression::garbage(op.span);
                    (
                        Type::Any,
                        Some(ParseError::UnsupportedOperationLHS(
                            "less-than or equal comparison".into(),
                            op.span,
                            lhs.span,
                            lhs.ty.clone(),
                        )),
                    )
                }
            },
            Operator::Comparison(Comparison::GreaterThan) => match (&lhs.ty, &rhs.ty) {
                (Type::Int, Type::Int) => (Type::Bool, None),
                (Type::Float, Type::Int) => (Type::Bool, None),
                (Type::Int, Type::Float) => (Type::Bool, None),
                (Type::Float, Type::Float) => (Type::Bool, None),
                (Type::Duration, Type::Duration) => (Type::Bool, None),
                (Type::Filesize, Type::Filesize) => (Type::Bool, None),

                (Type::Custom(a), Type::Custom(b)) if a == b => (Type::Custom(a.to_string()), None),
                (Type::Custom(a), _) => (Type::Custom(a.to_string()), None),

                (Type::Any, _) => (Type::Bool, None),
                (_, Type::Any) => (Type::Bool, None),

                (Type::Nothing, _) => (Type::Nothing, None),
                (_, Type::Nothing) => (Type::Nothing, None),
                (Type::Int | Type::Float | Type::Duration | Type::Filesize, _) => {
                    *op = Expression::garbage(op.span);
                    (
                        Type::Any,
                        Some(ParseError::UnsupportedOperationRHS(
                            "greater-than comparison".into(),
                            op.span,
                            lhs.span,
                            lhs.ty.clone(),
                            rhs.span,
                            rhs.ty.clone(),
                        )),
                    )
                }
                _ => {
                    *op = Expression::garbage(op.span);
                    (
                        Type::Any,
                        Some(ParseError::UnsupportedOperationLHS(
                            "greater-than comparison".into(),
                            op.span,
                            lhs.span,
                            lhs.ty.clone(),
                        )),
                    )
                }
            },
            Operator::Comparison(Comparison::GreaterThanOrEqual) => match (&lhs.ty, &rhs.ty) {
                (Type::Int, Type::Int) => (Type::Bool, None),
                (Type::Float, Type::Int) => (Type::Bool, None),
                (Type::Int, Type::Float) => (Type::Bool, None),
                (Type::Float, Type::Float) => (Type::Bool, None),
                (Type::Duration, Type::Duration) => (Type::Bool, None),
                (Type::Filesize, Type::Filesize) => (Type::Bool, None),

                (Type::Custom(a), Type::Custom(b)) if a == b => (Type::Custom(a.to_string()), None),
                (Type::Custom(a), _) => (Type::Custom(a.to_string()), None),

                (Type::Any, _) => (Type::Bool, None),
                (_, Type::Any) => (Type::Bool, None),

                (Type::Nothing, _) => (Type::Nothing, None),
                (_, Type::Nothing) => (Type::Nothing, None),
                (Type::Int | Type::Float | Type::Duration | Type::Filesize, _) => {
                    *op = Expression::garbage(op.span);
                    (
                        Type::Any,
                        Some(ParseError::UnsupportedOperationRHS(
                            "greater-than or equal comparison".into(),
                            op.span,
                            lhs.span,
                            lhs.ty.clone(),
                            rhs.span,
                            rhs.ty.clone(),
                        )),
                    )
                }
                _ => {
                    *op = Expression::garbage(op.span);
                    (
                        Type::Any,
                        Some(ParseError::UnsupportedOperationLHS(
                            "greater-than or equal comparison".into(),
                            op.span,
                            lhs.span,
                            lhs.ty.clone(),
                        )),
                    )
                }
            },
            Operator::Comparison(Comparison::Equal) => match (&lhs.ty, &rhs.ty) {
                (Type::Custom(a), Type::Custom(b)) if a == b => (Type::Custom(a.to_string()), None),
                (Type::Custom(a), _) => (Type::Custom(a.to_string()), None),

                _ => (Type::Bool, None),
            },
            Operator::Comparison(Comparison::NotEqual) => match (&lhs.ty, &rhs.ty) {
                (Type::Custom(a), Type::Custom(b)) if a == b => (Type::Custom(a.to_string()), None),
                (Type::Custom(a), _) => (Type::Custom(a.to_string()), None),

                _ => (Type::Bool, None),
            },
            Operator::Comparison(Comparison::RegexMatch) => match (&lhs.ty, &rhs.ty) {
                (Type::String, Type::String) => (Type::Bool, None),
                (Type::Any, _) => (Type::Bool, None),
                (_, Type::Any) => (Type::Bool, None),

                (Type::Custom(a), Type::Custom(b)) if a == b => (Type::Custom(a.to_string()), None),
                (Type::Custom(a), _) => (Type::Custom(a.to_string()), None),

                (Type::String, _) => {
                    *op = Expression::garbage(op.span);
                    (
                        Type::Any,
                        Some(ParseError::UnsupportedOperationRHS(
                            "regex matching".into(),
                            op.span,
                            lhs.span,
                            lhs.ty.clone(),
                            rhs.span,
                            rhs.ty.clone(),
                        )),
                    )
                }
                _ => {
                    *op = Expression::garbage(op.span);
                    (
                        Type::Any,
                        Some(ParseError::UnsupportedOperationLHS(
                            "regex matching".into(),
                            op.span,
                            lhs.span,
                            lhs.ty.clone(),
                        )),
                    )
                }
            },
            Operator::Comparison(Comparison::NotRegexMatch) => match (&lhs.ty, &rhs.ty) {
                (Type::String, Type::String) => (Type::Bool, None),
                (Type::Any, _) => (Type::Bool, None),
                (_, Type::Any) => (Type::Bool, None),

                (Type::Custom(a), Type::Custom(b)) if a == b => (Type::Custom(a.to_string()), None),
                (Type::Custom(a), _) => (Type::Custom(a.to_string()), None),

                (Type::String, _) => {
                    *op = Expression::garbage(op.span);
                    (
                        Type::Any,
                        Some(ParseError::UnsupportedOperationRHS(
                            "regex matching".into(),
                            op.span,
                            lhs.span,
                            lhs.ty.clone(),
                            rhs.span,
                            rhs.ty.clone(),
                        )),
                    )
                }
                _ => {
                    *op = Expression::garbage(op.span);
                    (
                        Type::Any,
                        Some(ParseError::UnsupportedOperationLHS(
                            "regex matching".into(),
                            op.span,
                            lhs.span,
                            lhs.ty.clone(),
                        )),
                    )
                }
            },
            Operator::Comparison(Comparison::StartsWith) => match (&lhs.ty, &rhs.ty) {
                (Type::String, Type::String) => (Type::Bool, None),
                (Type::Any, _) => (Type::Bool, None),
                (_, Type::Any) => (Type::Bool, None),

                (Type::Custom(a), Type::Custom(b)) if a == b => (Type::Custom(a.to_string()), None),
                (Type::Custom(a), _) => (Type::Custom(a.to_string()), None),

                (Type::String, _) => {
                    *op = Expression::garbage(op.span);
                    (
                        Type::Any,
                        Some(ParseError::UnsupportedOperationRHS(
                            "starts-with comparison".into(),
                            op.span,
                            lhs.span,
                            lhs.ty.clone(),
                            rhs.span,
                            rhs.ty.clone(),
                        )),
                    )
                }
                _ => {
                    *op = Expression::garbage(op.span);
                    (
                        Type::Any,
                        Some(ParseError::UnsupportedOperationLHS(
                            "starts-with comparison".into(),
                            op.span,
                            lhs.span,
                            lhs.ty.clone(),
                        )),
                    )
                }
            },
            Operator::Comparison(Comparison::EndsWith) => match (&lhs.ty, &rhs.ty) {
                (Type::String, Type::String) => (Type::Bool, None),
                (Type::Any, _) => (Type::Bool, None),
                (_, Type::Any) => (Type::Bool, None),

                (Type::Custom(a), Type::Custom(b)) if a == b => (Type::Custom(a.to_string()), None),
                (Type::Custom(a), _) => (Type::Custom(a.to_string()), None),

                (Type::String, _) => {
                    *op = Expression::garbage(op.span);
                    (
                        Type::Any,
                        Some(ParseError::UnsupportedOperationRHS(
                            "ends-with comparison".into(),
                            op.span,
                            lhs.span,
                            lhs.ty.clone(),
                            rhs.span,
                            rhs.ty.clone(),
                        )),
                    )
                }
                _ => {
                    *op = Expression::garbage(op.span);
                    (
                        Type::Any,
                        Some(ParseError::UnsupportedOperationLHS(
                            "ends-with comparison".into(),
                            op.span,
                            lhs.span,
                            lhs.ty.clone(),
                        )),
                    )
                }
            },
            Operator::Comparison(Comparison::In) => match (&lhs.ty, &rhs.ty) {
                (t, Type::List(u)) if type_compatible(t, u) => (Type::Bool, None),
                (Type::Int | Type::Float, Type::Range) => (Type::Bool, None),
                (Type::String, Type::String) => (Type::Bool, None),
                (Type::String, Type::Record(_)) => (Type::Bool, None),

                (Type::Custom(a), Type::Custom(b)) if a == b => (Type::Custom(a.to_string()), None),
                (Type::Custom(a), _) => (Type::Custom(a.to_string()), None),

                (Type::Any, _) => (Type::Bool, None),
                (_, Type::Any) => (Type::Bool, None),
                (Type::Int | Type::Float | Type::String, _) => {
                    *op = Expression::garbage(op.span);
                    (
                        Type::Any,
                        Some(ParseError::UnsupportedOperationRHS(
                            "subset comparison".into(),
                            op.span,
                            lhs.span,
                            lhs.ty.clone(),
                            rhs.span,
                            rhs.ty.clone(),
                        )),
                    )
                }
                _ => {
                    *op = Expression::garbage(op.span);
                    (
                        Type::Any,
                        Some(ParseError::UnsupportedOperationLHS(
                            "subset comparison".into(),
                            op.span,
                            lhs.span,
                            lhs.ty.clone(),
                        )),
                    )
                }
            },
            Operator::Comparison(Comparison::NotIn) => match (&lhs.ty, &rhs.ty) {
                (t, Type::List(u)) if type_compatible(t, u) => (Type::Bool, None),
                (Type::Int | Type::Float, Type::Range) => (Type::Bool, None),
                (Type::String, Type::String) => (Type::Bool, None),
                (Type::String, Type::Record(_)) => (Type::Bool, None),

                (Type::Custom(a), Type::Custom(b)) if a == b => (Type::Custom(a.to_string()), None),
                (Type::Custom(a), _) => (Type::Custom(a.to_string()), None),

                (Type::Any, _) => (Type::Bool, None),
                (_, Type::Any) => (Type::Bool, None),
                (Type::Int | Type::Float | Type::String, _) => {
                    *op = Expression::garbage(op.span);
                    (
                        Type::Any,
                        Some(ParseError::UnsupportedOperationRHS(
                            "subset comparison".into(),
                            op.span,
                            lhs.span,
                            lhs.ty.clone(),
                            rhs.span,
                            rhs.ty.clone(),
                        )),
                    )
                }
                _ => {
                    *op = Expression::garbage(op.span);
                    (
                        Type::Any,
                        Some(ParseError::UnsupportedOperationLHS(
                            "subset comparison".into(),
                            op.span,
                            lhs.span,
                            lhs.ty.clone(),
                        )),
                    )
                }
            },
            Operator::Bits(Bits::ShiftLeft)
            | Operator::Bits(Bits::ShiftRight)
            | Operator::Bits(Bits::BitOr)
            | Operator::Bits(Bits::BitXor)
            | Operator::Bits(Bits::BitAnd) => match (&lhs.ty, &rhs.ty) {
                (Type::Int, Type::Int) => (Type::Int, None),

                (Type::Any, _) => (Type::Any, None),
                (_, Type::Any) => (Type::Any, None),
                (Type::Int, _) => {
                    *op = Expression::garbage(op.span);
                    (
                        Type::Any,
                        Some(ParseError::UnsupportedOperationRHS(
                            "bit operations".into(),
                            op.span,
                            lhs.span,
                            lhs.ty.clone(),
                            rhs.span,
                            rhs.ty.clone(),
                        )),
                    )
                }
                _ => {
                    *op = Expression::garbage(op.span);
                    (
                        Type::Any,
                        Some(ParseError::UnsupportedOperationLHS(
                            "bit operations".into(),
                            op.span,
                            lhs.span,
                            lhs.ty.clone(),
                        )),
                    )
                }
            },
            Operator::Assignment(_) => match (&lhs.ty, &rhs.ty) {
                (x, y) if x == y => (Type::Nothing, None),
                (Type::Any, _) => (Type::Nothing, None),
                (_, Type::Any) => (Type::Nothing, None),
                (Type::List(_), Type::List(_)) => (Type::Nothing, None),
                (x, y) => (
                    Type::Nothing,
                    Some(ParseError::Mismatch(x.to_string(), y.to_string(), rhs.span)),
                ),
            },
        },
        _ => {
            *op = Expression::garbage(op.span);

            (
                Type::Any,
                Some(ParseError::IncompleteMathExpression(op.span)),
            )
        }
    }
}
