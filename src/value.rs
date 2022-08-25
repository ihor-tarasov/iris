use std::{
    hash::{Hash, Hasher},
    marker::PhantomData,
};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Value {
    Bool(bool),
    Integer(i64),
    Real(f64),
}

impl Eq for Value {}

impl Hash for Value {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match *self {
            Value::Bool(value) => value.hash(state),
            Value::Integer(value) => value.hash(state),
            Value::Real(value) => value.to_bits().hash(state),
        }
    }
}

pub type OperatorResult = Result<Value, String>;

pub trait BinaryOperator {
    fn eval(lhs: Value, rhs: Value) -> OperatorResult;
}

pub trait IntOperator {
    fn eval(lhs: i64, rhs: i64) -> OperatorResult;
}

pub trait RealOperator {
    fn eval(lhs: f64, rhs: f64) -> OperatorResult;
}

pub trait BoolOperator {
    fn eval(lhs: bool, rhs: bool) -> OperatorResult;
}

macro_rules! generate_implement {
    ($trait_name:ident, $struct_name:ident, $value_type:ident, $result_type:ident, $op:tt) => {
        impl $trait_name for $struct_name {
            fn eval(lhs: $value_type, rhs: $value_type) -> OperatorResult {
                Ok(Value::$result_type(lhs $op rhs))
            }
        }
    };
}

pub struct And;
pub struct Or;
pub struct Xor;

generate_implement!(IntOperator, And, i64, Integer, &);
generate_implement!(IntOperator, Or, i64, Integer, |);
generate_implement!(IntOperator, Xor, i64, Integer, ^);

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

impl IntOperator for Addict {
    fn eval(lhs: i64, rhs: i64) -> OperatorResult {
        Ok(Value::Integer(lhs.wrapping_add(rhs)))
    }
}

generate_implement!(RealOperator, Addict, f64, Real, +);
generate_implement!(RealOperator, Subtract, f64, Real, -);
generate_implement!(RealOperator, Multiply, f64, Real, *);
generate_implement!(RealOperator, Divide, f64, Real, /);
generate_implement!(RealOperator, Modulo, f64, Real, %);

pub struct Subtract;

impl IntOperator for Subtract {
    fn eval(lhs: i64, rhs: i64) -> OperatorResult {
        Ok(Value::Integer(lhs.wrapping_sub(rhs)))
    }
}

pub struct Multiply;

impl IntOperator for Multiply {
    fn eval(lhs: i64, rhs: i64) -> OperatorResult {
        Ok(Value::Integer(lhs.wrapping_mul(rhs)))
    }
}

pub struct Divide;

impl IntOperator for Divide {
    fn eval(lhs: i64, rhs: i64) -> OperatorResult {
        if rhs == 0 {
            Err(format!("Dividing by zero."))
        } else {
            Ok(Value::Integer(lhs.wrapping_div(rhs)))
        }
    }
}

pub struct Modulo;

impl IntOperator for Modulo {
    fn eval(lhs: i64, rhs: i64) -> OperatorResult {
        if rhs == 0 {
            Err(format!("Dividing by zero."))
        } else {
            Ok(Value::Integer(lhs % rhs))
        }
    }
}

pub struct Less;
pub struct LessEqual;
pub struct Greater;
pub struct GreaterEqual;

generate_implement!(IntOperator, Less, i64, Bool, <);
generate_implement!(RealOperator, Less, f64, Bool, <);
generate_implement!(IntOperator, LessEqual, i64, Bool, <=);
generate_implement!(RealOperator, LessEqual, f64, Bool, <=);
generate_implement!(IntOperator, Greater, i64, Bool, >);
generate_implement!(RealOperator, Greater, f64, Bool, >);
generate_implement!(IntOperator, GreaterEqual, i64, Bool, >=);
generate_implement!(RealOperator, GreaterEqual, f64, Bool, >=);

pub struct Equal;
pub struct NotEqual;

generate_implement!(BoolOperator, Equal, bool, Bool, ==);
generate_implement!(IntOperator, Equal, i64, Bool, ==);
generate_implement!(RealOperator, Equal, f64, Bool, ==);
generate_implement!(BoolOperator, NotEqual, bool, Bool, !=);
generate_implement!(IntOperator, NotEqual, i64, Bool, !=);
generate_implement!(RealOperator, NotEqual, f64, Bool, !=);

fn unable_to_use(lhs: Value, rhs: Value) -> OperatorResult {
    Err(format!(
        "Can't use {:?} and {:?} in this binary operation.",
        lhs, rhs,
    ))
}

pub struct ArithmeticOrComparison<T: IntOperator + RealOperator> {
    phantom: PhantomData<T>,
}

impl<T: IntOperator + RealOperator> BinaryOperator for ArithmeticOrComparison<T> {
    fn eval(lhs: Value, rhs: Value) -> OperatorResult {
        match (lhs, rhs) {
            (Value::Integer(lhs), Value::Integer(rhs)) => <T as IntOperator>::eval(lhs, rhs),
            (Value::Integer(lhs), Value::Real(rhs)) => <T as RealOperator>::eval(lhs as f64, rhs),
            (Value::Real(lhs), Value::Integer(rhs)) => <T as RealOperator>::eval(lhs, rhs as f64),
            (Value::Real(lhs), Value::Real(rhs)) => <T as RealOperator>::eval(lhs, rhs),
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

pub struct Equality<T: BoolOperator + IntOperator + RealOperator> {
    phantom: PhantomData<T>,
}

impl<T: BoolOperator + IntOperator + RealOperator> BinaryOperator for Equality<T> {
    fn eval(lhs: Value, rhs: Value) -> OperatorResult {
        match (lhs, rhs) {
            (Value::Bool(lhs), Value::Bool(rhs)) => <T as BoolOperator>::eval(lhs, rhs),
            (Value::Integer(lhs), Value::Integer(rhs)) => <T as IntOperator>::eval(lhs, rhs),
            (Value::Integer(lhs), Value::Real(rhs)) => <T as RealOperator>::eval(lhs as f64, rhs),
            (Value::Real(lhs), Value::Integer(rhs)) => <T as RealOperator>::eval(lhs, rhs as f64),
            (Value::Real(lhs), Value::Real(rhs)) => <T as RealOperator>::eval(lhs, rhs),
            _ => unable_to_use(lhs, rhs),
        }
    }
}
