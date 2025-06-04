//! Generic input system.
//!
//! # Components
//! - `GenericInput<T: InputType>`: A generic wrapper for input components. 
//! - Concrete Inputs: Implement `InputType` to provide typed input handling.
//! - Form: Composes multiple `GenericInput` fields and handles validation.
//! # Errors
//! Are handled on three levels: type level, field level and entity level.
//! ## Type level
//! - `InputType`s are typed which makes it so that they cannot work if an input is not parseable.
//!     - This makes sense for simple inputs that parse user-inputted text.
//!     - More complex input types (such as `Selector` or `SubForm`) don't allow invalid inputs in the first place because values are not parsed from user-typed strings.
//! - Due to this constraint, the user cannot type invalid types (for instance, typing letters in a number input field won't change the input value at all).
//! - This is managed internally in an InputType, so the behavior may be adjusted based on the needs.
//! - Errors of this level are not supposed to be printed, so there's no API that allows doing it directly.
//! ## Field level
//! - InputType contains a `validate` method for a field-level error validation
//! ## Form level
//! - GenericInput accepts `error` in props for an entity-level validation
//! - The form should provide this error in case input is invalid
//! 

pub mod form;
pub mod inputs;
pub mod generic_input;
pub mod input_result;
