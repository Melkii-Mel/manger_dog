use crate::pre_built::regexes;
use actix_surreal_starter_macros::{impl_display_for_error, impl_validators};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error, Clone, Serialize, Deserialize)]
pub enum ValidationError {
    StringIsEmpty,
    StringTooShort,
    StringTooLong,
    GTZero,
    GEZero,
    LTZero,
    LEZero,
    EQZero,
    NEZero,
    V1LTV2,
    V1LEV2,
    V1GTV2,
    V1GEV2,
    V1EQV2,
    V1NEV2,
    EmailFormatInvalid,
    PasswordTooShort,
    PasswordTooLong,
    PasswordMustNotContainSpaces,
    PasswordContainsInvalidCharacters,
    PasswordMustContainUppercase,
    PasswordMustContainLowercase,
    PasswordMustContainDigit,
    PasswordMustContainSpecial,
    ValueIsSome,
    ValueIsNone,
}

impl_display_for_error!(ValidationError);

impl_validators! {
    (trait_name: pub DefaultValidations, parameter_name: v, error: ValidationError) {
        not_empty(&String) {
            v.trim().len() == 0 => StringIsEmpty
        }
        gt_zero<T: PartialOrd|Default>(&T) {
            *v <= T::default() => LEZero
        }
        ge_zero<T: PartialOrd|Default>(&T) {
            *v < T::default() => LTZero
        }
        lt_zero<T: PartialOrd|Default>(&T) {
            *v >= T::default() => GEZero
        }
        le_zero<T: PartialOrd|Default>(&T) {
            *v > T::default() => GTZero
        }
        eq_zero<T: PartialOrd|Default>(&T) {
            *v > T::default() => EQZero
        }
        ne_zero<T: PartialOrd|Default>(&T) {
            *v > T::default() => NEZero
        }
        length_at_least(&String, usize) {
            v.0.chars().count() < v.1 => StringTooShort
        }
        length_at_most(&String, usize) {
            v.0.chars().count() > v.1 => StringTooLong
        }
        length_in_range(&String, usize, usize) {
            match v.0.chars().count() {
                c if c < v.1 => Err(ValidationError::StringTooShort),
                c if c > v.2 => Err(ValidationError::StringTooLong),
                _ => Ok(()),
            }
        }
        v1_ge_v2<T: PartialOrd>(&T, &T) {
            v.0 < v.1 => V1LTV2
        }
        v1_gt_v2<T: PartialOrd>(&T, &T) {
            v.0 <= v.1 => V1LEV2
        }
        v1_le_v2<T: PartialOrd>(&T, &T) {
            v.0 > v.1 => V1GTV2
        }
        v1_lt_v2<T: PartialOrd>(&T, &T) {
            v.0 >= v.1 => V1GEV2
        }
        v1_eq_v2<T: PartialOrd>(&T, &T) {
            v.0 >= v.1 => V1EQV2
        }
        v1_ne_v2<T: PartialOrd>(&T, &T) {
            v.0 >= v.1 => V1NEV2
        }
        email_format(&String) {
            !regexes::EMAIL.is_match(v) => EmailFormatInvalid
        }
        password_length(&String, usize, usize) {
            Self::length_in_range((v.0, v.1, v.2)).map_err(|e| match e {
                ValidationError::StringTooShort => ValidationError::PasswordTooShort,
                _ => ValidationError::PasswordTooLong
            })
        }
        password_basic(&String) {
            Self::password_length((v, 6, 64))
            regexes::SPACES.is_match(v) => PasswordMustNotContainSpaces
            !regexes::VALID_PASSWORD_CHARS.is_match(v) => PasswordContainsInvalidCharacters
            !regexes::DIGIT.is_match(v) => PasswordMustContainDigit
        }
        password_moderate(&String) {
            Self::password_basic(v)
            Self::password_length((v, 8, 64))
            !regexes::UPPERCASE.is_match(v) => PasswordMustContainUppercase
            !regexes::LOWERCASE.is_match(v) => PasswordMustContainLowercase
        }
        password_strict(&String) {
            Self::password_moderate(v)
            Self::password_length((v, 12, 64))
            !regexes::SPECIAL_PASSWORD_CHAR.is_match(v) => PasswordMustContainSpecial
        }
        none<T>(&Option<T>) {
            v.is_some() => ValueIsSome
        }
        some<T>(&Option<T>) {
            v.is_none() => ValueIsNone
        }
        optional_v2_gt_v1<T: PartialOrd>(&T, &Option<T>) {
            match v.1 {
                Some(value) => Self::v1_le_v2((v.0, value)),
                None => Ok(()),
            }
        }
    }
}
