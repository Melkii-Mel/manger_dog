macro_rules! map_var_err {
    ($env_option:expr, $env_name:expr) => {
        $env_option.as_ref().map_err(|e| {
            io::Error::new(
                ErrorKind::Other,
                format!(
                    "Environment variable {} not found. Internal error: {:?}",
                    $env_name, e
                ),
            )
        })
    };
}

macro_rules! to_arc {
    ($value:ident) => {
        let $value = Arc::new($value);
    };
    ($($value:ident),*$(,)*) => {
        $(
            let $value = Arc::new($value);
        )*
    };
}

macro_rules! builder {
    ($($argument:ident: $ty:ty),*$(,)*) => {
        builder!($($argument: $ty => $argument,)*)
    };
    ($($argument:ident: $ty:ty => $logic: expr),*$(,)*) => {
        $(
        pub fn $argument(mut self, $argument: $ty) -> Self {
            self.$argument = Some($logic);
            self
        }
        )*
    };
}

macro_rules! enclose {
    (($($arc:ident),*) $closure:expr) => {
        {
            $(let $arc = $arc.clone();)*
            $closure
        }
    };
}
