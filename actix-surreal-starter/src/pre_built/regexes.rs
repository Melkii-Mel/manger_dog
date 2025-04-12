use std::sync::LazyLock;
use regex::Regex;

macro_rules! regexes {
    {$($name:ident => $value:literal)*} => {
        $(
        pub static $name: LazyLock<Regex> = LazyLock::new(|| Regex::new($value).unwrap());
        )*
    };
}

regexes!{
    EMAIL => r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$"
    SPACES => r"\s"
    VALID_PASSWORD_CHARS => r"^[a-zA-Z0-9@#!$%^&*()_+\-=<>?{}[\]|~]+$"
    UPPERCASE => r"[A-Z]"
    LOWERCASE => r"[a-z]"
    DIGIT => r"\d"
    SPECIAL_PASSWORD_CHAR => r"[^\w\s:]"
}

