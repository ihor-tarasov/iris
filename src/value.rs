#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Value {
    Integer(i64),
}

pub type OperatorResult = Result<Value, String>;

pub trait BinaryOperator {
    fn eval(lhs: Value, rhs: Value) -> OperatorResult;
}

pub trait IntRealOperator {
    fn eval_int(lhs: i64, rhs: i64) -> OperatorResult;
}

pub struct Addict;

impl IntRealOperator for Addict {
    fn eval_int(lhs: i64, rhs: i64) -> OperatorResult {
        Ok(Value::Integer(lhs.wrapping_add(rhs)))
    }
}

pub struct Multiply;

impl IntRealOperator for Multiply {
    fn eval_int(lhs: i64, rhs: i64) -> OperatorResult {
        Ok(Value::Integer(lhs.wrapping_mul(rhs)))
    }
}

pub struct Arithmetic<T: IntRealOperator> {
    phantom: std::marker::PhantomData<T>,
}

impl<T: IntRealOperator> BinaryOperator for Arithmetic<T> {
    fn eval(lhs: Value, rhs: Value) -> OperatorResult {
        match (lhs, rhs) {
            (Value::Integer(lhs), Value::Integer(rhs)) => T::eval_int(lhs, rhs),
        }
    }
}
