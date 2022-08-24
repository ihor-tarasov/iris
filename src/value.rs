use std::{
    fmt,
    hash::{Hash, Hasher},
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
    Integer(i64),
    Real(Real),
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Value::Integer(value) => write!(f, "{}", value),
            Value::Real(value) => write!(f, "{}", value.0),
        }
    }
}

pub type OperatorResult = Result<Value, String>;

pub trait BinaryOperator {
    fn eval(lhs: Value, rhs: Value) -> OperatorResult;
}

pub trait IntRealOperator {
    fn eval_int(lhs: i64, rhs: i64) -> OperatorResult;
    fn eval_real(lhs: f64, rhs: f64) -> OperatorResult;
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

pub struct Arithmetic<T: IntRealOperator> {
    phantom: std::marker::PhantomData<T>,
}

impl<T: IntRealOperator> BinaryOperator for Arithmetic<T> {
    fn eval(lhs: Value, rhs: Value) -> OperatorResult {
        match (lhs, rhs) {
            (Value::Integer(lhs), Value::Integer(rhs)) => T::eval_int(lhs, rhs),
            (Value::Integer(lhs), Value::Real(rhs)) => T::eval_real(lhs as f64, rhs.0),
            (Value::Real(lhs), Value::Integer(rhs)) => T::eval_real(lhs.0, rhs as f64),
            (Value::Real(lhs), Value::Real(rhs)) => T::eval_real(lhs.0, rhs.0),
        }
    }
}
