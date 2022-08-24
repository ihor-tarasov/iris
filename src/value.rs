use std::{
    fmt,
    hash::{Hash, Hasher},
    marker::PhantomData,
};

#[derive(Clone, Copy, PartialEq)]
pub struct Real(pub f64);

impl Eq for Real {}

impl Hash for Real {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.to_bits().hash(state)
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum Value {
    Bool(bool),
    Integer(i64),
    Real(Real),
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Value::Bool(value) => write!(f, "{}", value),
            Value::Integer(value) => write!(f, "{}", value),
            Value::Real(value) => write!(f, "{}", value.0),
        }
    }
}

fn type_name(value: Value) -> &'static str {
    match value {
        Value::Bool(_) => "bool",
        Value::Integer(_) => "int",
        Value::Real(_) => "real",
    }
}

pub type OperatorResult = Result<Value, String>;

pub trait BinaryOperator {
    fn eval(lhs: Value, rhs: Value) -> OperatorResult;
}

pub trait IntOperator {
    fn eval(lhs: i64, rhs: i64) -> OperatorResult;
}

pub trait IntRealOperator {
    fn eval_int(lhs: i64, rhs: i64) -> OperatorResult;
    fn eval_real(lhs: f64, rhs: f64) -> OperatorResult;
}

pub trait BoolIntRealOperator {
    fn eval_bool(lhs: bool, rhs: bool) -> OperatorResult;
    fn eval_int(lhs: i64, rhs: i64) -> OperatorResult;
    fn eval_real(lhs: f64, rhs: f64) -> OperatorResult;
}

pub struct And;

impl IntOperator for And {
    fn eval(lhs: i64, rhs: i64) -> OperatorResult {
        Ok(Value::Integer(lhs & rhs))
    }
}

pub struct Or;

impl IntOperator for Or {
    fn eval(lhs: i64, rhs: i64) -> OperatorResult {
        Ok(Value::Integer(lhs | rhs))
    }
}

pub struct Xor;

impl IntOperator for Xor {
    fn eval(lhs: i64, rhs: i64) -> OperatorResult {
        Ok(Value::Integer(lhs ^ rhs))
    }
}

fn correct_rhs_for_shirt(rhs: i64) -> Result<u32, String> {
    if rhs < 0 {
        Err(format!(
            "Unable to use negative ({}) value as right hand side in shift operation.",
            rhs
        ))
    } else {
        if rhs as usize > u32::MAX as usize {
            Err(format!(
                "Unable to use so big ({}) value as right hand side in shift operation.",
                rhs
            ))
        } else {
            Ok(rhs as u32)
        }
    }
}

pub struct Shl;

impl IntOperator for Shl {
    fn eval(lhs: i64, rhs: i64) -> OperatorResult {
        Ok(Value::Integer(
            lhs.wrapping_shl(correct_rhs_for_shirt(rhs)?),
        ))
    }
}

pub struct Shr;

impl IntOperator for Shr {
    fn eval(lhs: i64, rhs: i64) -> OperatorResult {
        Ok(Value::Integer(
            lhs.wrapping_shr(correct_rhs_for_shirt(rhs)?),
        ))
    }
}

pub struct Addict;

impl IntRealOperator for Addict {
    fn eval_int(lhs: i64, rhs: i64) -> OperatorResult {
        Ok(Value::Integer(lhs.wrapping_add(rhs)))
    }

    fn eval_real(lhs: f64, rhs: f64) -> OperatorResult {
        Ok(Value::Real(Real(lhs + rhs)))
    }
}

pub struct Subtract;

impl IntRealOperator for Subtract {
    fn eval_int(lhs: i64, rhs: i64) -> OperatorResult {
        Ok(Value::Integer(lhs.wrapping_sub(rhs)))
    }

    fn eval_real(lhs: f64, rhs: f64) -> OperatorResult {
        Ok(Value::Real(Real(lhs - rhs)))
    }
}

pub struct Multiply;

impl IntRealOperator for Multiply {
    fn eval_int(lhs: i64, rhs: i64) -> OperatorResult {
        Ok(Value::Integer(lhs.wrapping_mul(rhs)))
    }

    fn eval_real(lhs: f64, rhs: f64) -> OperatorResult {
        Ok(Value::Real(Real(lhs * rhs)))
    }
}

pub struct Divide;

impl IntRealOperator for Divide {
    fn eval_int(lhs: i64, rhs: i64) -> OperatorResult {
        if rhs == 0 {
            Err(format!("Dividing by zero."))
        } else {
            Ok(Value::Integer(lhs.wrapping_div(rhs)))
        }
    }

    fn eval_real(lhs: f64, rhs: f64) -> OperatorResult {
        Ok(Value::Real(Real(lhs / rhs)))
    }
}

pub struct Modulo;

impl IntRealOperator for Modulo {
    fn eval_int(lhs: i64, rhs: i64) -> OperatorResult {
        if rhs == 0 {
            Err(format!("Dividing by zero."))
        } else {
            Ok(Value::Integer(lhs % rhs))
        }
    }

    fn eval_real(lhs: f64, rhs: f64) -> OperatorResult {
        Ok(Value::Real(Real(lhs % rhs)))
    }
}

pub struct Less;

impl IntRealOperator for Less {
    fn eval_int(lhs: i64, rhs: i64) -> OperatorResult {
        Ok(Value::Bool(lhs < rhs))
    }

    fn eval_real(lhs: f64, rhs: f64) -> OperatorResult {
        Ok(Value::Bool(lhs < rhs))
    }
}

pub struct Greater;

impl IntRealOperator for Greater {
    fn eval_int(lhs: i64, rhs: i64) -> OperatorResult {
        Ok(Value::Bool(lhs > rhs))
    }

    fn eval_real(lhs: f64, rhs: f64) -> OperatorResult {
        Ok(Value::Bool(lhs > rhs))
    }
}

pub struct LessEqual;

impl IntRealOperator for LessEqual {
    fn eval_int(lhs: i64, rhs: i64) -> OperatorResult {
        Ok(Value::Bool(lhs <= rhs))
    }

    fn eval_real(lhs: f64, rhs: f64) -> OperatorResult {
        Ok(Value::Bool(lhs <= rhs))
    }
}

pub struct GreaterEqual;

impl IntRealOperator for GreaterEqual {
    fn eval_int(lhs: i64, rhs: i64) -> OperatorResult {
        Ok(Value::Bool(lhs >= rhs))
    }

    fn eval_real(lhs: f64, rhs: f64) -> OperatorResult {
        Ok(Value::Bool(lhs >= rhs))
    }
}

pub struct Equal;

impl BoolIntRealOperator for Equal {
    fn eval_bool(lhs: bool, rhs: bool) -> OperatorResult {
        Ok(Value::Bool(lhs == rhs))
    }

    fn eval_int(lhs: i64, rhs: i64) -> OperatorResult {
        Ok(Value::Bool(lhs == rhs))
    }

    fn eval_real(lhs: f64, rhs: f64) -> OperatorResult {
        Ok(Value::Bool(lhs == rhs))
    }
}

pub struct NotEqual;

impl BoolIntRealOperator for NotEqual {
    fn eval_bool(lhs: bool, rhs: bool) -> OperatorResult {
        Ok(Value::Bool(lhs != rhs))
    }

    fn eval_int(lhs: i64, rhs: i64) -> OperatorResult {
        Ok(Value::Bool(lhs != rhs))
    }

    fn eval_real(lhs: f64, rhs: f64) -> OperatorResult {
        Ok(Value::Bool(lhs != rhs))
    }
}

fn unable_to_use(lhs: Value, rhs: Value) -> OperatorResult {
    Err(format!(
        "Unable to use such types ({} and {}) in bitwise operation.",
        type_name(lhs),
        type_name(rhs)
    ))
}

pub struct ArithmeticOrComparison<T: IntRealOperator> {
    phantom: PhantomData<T>,
}

impl<T: IntRealOperator> BinaryOperator for ArithmeticOrComparison<T> {
    fn eval(lhs: Value, rhs: Value) -> OperatorResult {
        match (lhs, rhs) {
            (Value::Integer(lhs), Value::Integer(rhs)) => T::eval_int(lhs, rhs),
            (Value::Integer(lhs), Value::Real(rhs)) => T::eval_real(lhs as f64, rhs.0),
            (Value::Real(lhs), Value::Integer(rhs)) => T::eval_real(lhs.0, rhs as f64),
            (Value::Real(lhs), Value::Real(rhs)) => T::eval_real(lhs.0, rhs.0),
            _ => unable_to_use(lhs, rhs),
        }
    }
}

pub struct Bitwise<T: IntOperator> {
    phantom: PhantomData<T>,
}

impl<T: IntOperator> BinaryOperator for Bitwise<T> {
    fn eval(lhs: Value, rhs: Value) -> OperatorResult {
        match (lhs, rhs) {
            (Value::Integer(lhs), Value::Integer(rhs)) => T::eval(lhs, rhs),
            _ => unable_to_use(lhs, rhs),
        }
    }
}

pub struct Equality<T: BoolIntRealOperator> {
    phantom: PhantomData<T>,
}

impl<T: BoolIntRealOperator> BinaryOperator for Equality<T> {
    fn eval(lhs: Value, rhs: Value) -> OperatorResult {
        match (lhs, rhs) {
            (Value::Bool(lhs), Value::Bool(rhs)) => T::eval_bool(lhs, rhs),
            (Value::Integer(lhs), Value::Integer(rhs)) => T::eval_int(lhs, rhs),
            (Value::Integer(lhs), Value::Real(rhs)) => T::eval_real(lhs as f64, rhs.0),
            (Value::Real(lhs), Value::Integer(rhs)) => T::eval_real(lhs.0, rhs as f64),
            (Value::Real(lhs), Value::Real(rhs)) => T::eval_real(lhs.0, rhs.0),
            _ => unable_to_use(lhs, rhs),
        }
    }
}
