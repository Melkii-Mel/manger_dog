use crate::form_input::input_result::InputResult;
use actix_surreal_starter_types::ErrorEnum;
use derive_more::Display;
use thiserror::Error;

#[derive(Error, Debug, ErrorEnum, Display)]
pub enum RawNumberParseError {
    Empty,
    FormatInvalid,
    InvalidCharacters,
    MultipleDecimalPoints,
    MinusesInWrongPositions,
    WholeTooLarge,
    FracTooLarge,
    Incomplete,
}

#[derive(Error, Debug, ErrorEnum, Display)]
pub enum IntFormatError {
    NumberFormatError(#[from] RawNumberParseError),
    FracNotZero,
}

#[derive(Error, Debug, ErrorEnum, Display)]
pub enum MoneyFormatError {
    NumberFormatError(#[from] RawNumberParseError),
    IntFormatError(#[from] IntFormatError),
    FracTooLarge,
    TooLarge,
}

pub fn parse_number(input: &str) -> Result<(i64, i64, usize), RawNumberParseError> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err(RawNumberParseError::Empty);
    }

    if trimmed == "-" {
        return Err(RawNumberParseError::Incomplete);
    }

    let (negative, stripped) = if let Some(stripped) = trimmed.strip_prefix('-') {
        (true, stripped)
    } else {
        (false, trimmed)
    };

    if stripped.chars().any(|c| c.eq(&'-')) {
        return Err(RawNumberParseError::MinusesInWrongPositions);
    }

    let normalized = stripped.replace(',', ".");
    if !normalized.chars().all(|c| c.is_ascii_digit() || c == '.') {
        return Err(RawNumberParseError::InvalidCharacters);
    }

    let parts: Vec<&str> = normalized.split('.').collect();
    if parts.len() > 2 {
        return Err(RawNumberParseError::MultipleDecimalPoints);
    }

    let sign = if negative { -1 } else { 1 };
    let (frac, frac_len) = match parts.get(1) {
        Some(&digits) => {
            let padded = format!("{:0<1}", digits);
            (
                padded
                    .parse::<i64>()
                    .map_err(|_| RawNumberParseError::FracTooLarge)?,
                padded.len(),
            )
        }
        None => (0, 0usize),
    };

    Ok((
        sign * format!("{:0<1}", parts[0])
            .parse::<i64>()
            .map_err(|_| RawNumberParseError::WholeTooLarge)?,
        sign * frac,
        frac_len,
    ))
}

pub fn parse_int(input: &str) -> Result<i64, IntFormatError> {
    let (whole, frac, _) = parse_number(input)?;
    if frac != 0 {
        return Err(IntFormatError::FracNotZero);
    }
    Ok(whole)
}

pub fn parse_money(input: &str) -> Result<i64, MoneyFormatError> {
    let (whole, frac, frac_size) = parse_number(input)?;
    {
        if frac_size <= 2 {
            Ok(())
        } else {
            Err(MoneyFormatError::FracTooLarge)
        }
    }?;
    whole
        .checked_mul(100)
        .and_then(|w| w.checked_add(frac))
        .ok_or(MoneyFormatError::TooLarge)
}

pub fn parse_float(input: &str) -> Result<f64, RawNumberParseError> {
    let (whole, frac, frac_size) = parse_number(input)?;
    let frac = frac as f64;
    Ok(whole as f64 + frac / 10f64.powi(frac_size as i32))
}

macro_rules! impl_for_input_result {
    ($error_ident:ident |$e:ident| $logic:expr) => {
        impl<T> From<Result<T, $error_ident>> for InputResult<T> {
            fn from(value: Result<T, $error_ident>) -> Self {
                match value {
                    Ok(t) => InputResult::Ok(t),
                    Err($e) => $logic,
                }
            }
        }
    };
}

impl_for_input_result!(
    RawNumberParseError | e | {
        match e {
            RawNumberParseError::Empty => InputResult::Empty,
            RawNumberParseError::Incomplete => InputResult::Incomplete,
            RawNumberParseError::FormatInvalid
            | RawNumberParseError::InvalidCharacters
            | RawNumberParseError::MultipleDecimalPoints
            | RawNumberParseError::MinusesInWrongPositions
            | RawNumberParseError::WholeTooLarge
            | RawNumberParseError::FracTooLarge => InputResult::Err,
        }
    }
);

impl_for_input_result!(
    IntFormatError | e | {
        match e {
            IntFormatError::NumberFormatError(e) => Err(e).into(),
            IntFormatError::FracNotZero => InputResult::Err,
        }
    }
);

impl_for_input_result!(
    MoneyFormatError | e | {
        match e {
            MoneyFormatError::NumberFormatError(e) => Err(e).into(),
            MoneyFormatError::IntFormatError(e) => Err(e).into(),
            MoneyFormatError::FracTooLarge | MoneyFormatError::TooLarge => InputResult::Err,
        }
    }
);
