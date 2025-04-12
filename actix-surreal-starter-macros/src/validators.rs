#![allow(dead_code)]

use crate::validators::ValidationError::UsernameEmpty;

#[macro_export]
macro_rules! impl_validators {
    {
        (
            struct_name: $validation_struct_name:ident,
            parameter_name: $value_name:ident,
            error: $validation_error_type:ident $(,)?
        )
        {
            $(
                $fn_name:ident$(<$( $type_ident:ident $(: $( $generic_type:path )|* )? ),+>)?($($value_type:ty),*) {
                    $($logic:expr$(=>$result:ident $( ($result_expr:expr) )?)?)*
                }
            )*
        }
    } => {
        impl $validation_struct_name {
            $(
            pub fn $fn_name$(<$( $type_ident$(: $( $generic_type + )+ std::any::Any ,)* )?>)?($value_name:impl_validators!(@parse_param $($value_type)*)) -> Result<(), $validation_error_type> {
                $(
                impl_validators!(@parse_logic $logic $(, @error $result $( $result_expr )* => $validation_error_type)?);
                )*
                Ok(())
            }
            )*
        }
    };
    (@parse_logic $logic:expr, @error $result:ident $( $result_expr:expr )* => $validation_error_type:ident) => {
        if ($logic) {
            return Err($validation_error_type::$result $( ($result_expr) )*);
        }
    };
    (@parse_logic $logic:expr) => {
        $logic?;
    };
    (@parse_param $value_type:ty) => {
        $value_type
    };
    (@parse_param $($value_type:ty)*) => {
        ($($value_type,)*)
    };
}

// Example usage

enum ValidationError {
    UsernameEmpty,
    UsernameTooLong,
    InvalidPasswordFormat,
    StringEmpty,
    Date1NotLessThanDate2,
}

struct Validator;

impl_validators! {
    (struct_name: Validator, parameter_name: v, error: ValidationError) {
        not_empty(&String) {
            v.len() == 0 => StringEmpty
        }
        username(&String) {
            Self::not_empty(v).map_err(|_| UsernameEmpty)
            v.len() > 30 => UsernameTooLong
        }
        password(&String) {
            {
                v.contains("Ъ")
            } => InvalidPasswordFormat
        }
        date1_less_than_date2(&String, &String) {
            v.0.len() >= v.1.len() => Date1NotLessThanDate2
        }
    }
}
