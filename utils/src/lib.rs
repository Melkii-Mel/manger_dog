#[macro_export]
macro_rules! once_cell {
    ($vis:vis $struct_name:ident($cell_name:ident) { $( $field_name:ident: $type:ty ),*$(,)? }) => {
        static $cell_name: ::once_cell::sync::OnceCell<$struct_name> = ::once_cell::sync::OnceCell::new();
        #[derive(Default)]
        $vis struct $struct_name {
            $(
            $vis $field_name: $type
            )*
        }
        impl $struct_name {
            pub fn instance() -> &'static $struct_name {
                $cell_name.get().expect("Internal error: Instance is not initialized. Call Type::initialize(value:Type) at the start of the program")
            }
        
            pub fn initialize(value: $struct_name) {
                $cell_name.set(value).map_err(|_| "Instance is already initialized.").unwrap()
            }
        }
    };
}