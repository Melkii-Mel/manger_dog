#[macro_export]
macro_rules! try_parse_token {
    ($ps:ident $token:tt) => {
        if $ps.peek(::syn::Token![$token]) {
            $ps.parse::<::syn::Token![$token]>()?;
            true
        } else {
            false
        }
    };
}

#[macro_export]
macro_rules! force_parse_token {
    ($ps:ident $token:tt) => {
        if !try_parse_token!($ps $token) {
            Err(syn::Error::new($ps.cursor().span(), format!("`{}` expected", stringify!($token))))
        }
        else {
            Ok(())
        }
    };
}

#[macro_export]
macro_rules! parse_brackets {
    ($parser_ident:ident($ps:ident)) => {
        {
            let __content;
            syn::$parser_ident!(__content in $ps);
            __content
        }
    };
}