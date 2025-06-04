use std::ops::ControlFlow;
use std::ops::FromResidual;
use std::ops::Try;

#[derive(Debug, Clone)]
pub enum InputResult<T> {
    Ok(T),
    Err,
    Incomplete,
    Empty,
}

pub enum InputResultResidual {
    Err,
    Incomplete,
    Empty,
}

impl<T> Try for InputResult<T> {
    type Output = T;
    type Residual = InputResultResidual;

    fn from_output(output: T) -> Self {
        InputResult::Ok(output)
    }

    fn branch(self) -> ControlFlow<Self::Residual, T> {
        match self {
            InputResult::Ok(val) => ControlFlow::Continue(val),
            InputResult::Err => ControlFlow::Break(InputResultResidual::Err),
            InputResult::Incomplete => ControlFlow::Break(InputResultResidual::Incomplete),
            InputResult::Empty => ControlFlow::Break(InputResultResidual::Empty),
        }
    }
}

impl<T> FromResidual<InputResultResidual> for InputResult<T> {
    fn from_residual(residual: InputResultResidual) -> Self {
        match residual {
            InputResultResidual::Err => InputResult::Err,
            InputResultResidual::Incomplete => InputResult::Incomplete,
            InputResultResidual::Empty => InputResult::Empty,
        }
    }
}

impl<T> InputResult<T> {
    pub fn as_ref(&self) -> InputResult<&T> {
        match self {
            InputResult::Ok(ref val) => InputResult::Ok(val),
            InputResult::Err => InputResult::Err,
            InputResult::Incomplete => InputResult::Incomplete,
            InputResult::Empty => InputResult::Empty,
        }
    }
    pub fn map<U, F>(self, f: F) -> InputResult<U>
    where
        F: FnOnce(T) -> U,
    {
        match self {
            InputResult::Ok(val) => InputResult::Ok(f(val)),
            InputResult::Err => InputResult::Err,
            InputResult::Incomplete => InputResult::Incomplete,
            InputResult::Empty => InputResult::Empty,
        }
    }
    pub fn option(self) -> InputResult<Option<T>> {
        match self {
            InputResult::Ok(v) => InputResult::Ok(Some(v)),
            InputResult::Err => InputResult::Err,
            InputResult::Incomplete => InputResult::Incomplete,
            InputResult::Empty => InputResult::Ok(None),
        }
    }
    pub fn ok(self) -> Option<T> {
        match self {
            InputResult::Ok(v) => Some(v),
            _ => None
        }
    }
}
