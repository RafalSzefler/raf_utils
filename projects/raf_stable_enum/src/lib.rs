//! proc-macro project that holds [`macro@stable_enum`] attribute for generating
//! enums with stable ABI for cross binary usage.
#![warn(clippy::all, clippy::pedantic)]
#![allow(
    clippy::needless_return,
    clippy::redundant_field_names,
    clippy::unreadable_literal,
    clippy::inline_always,
    clippy::must_use_candidate,
    clippy::module_name_repetitions,
)]

mod input_reader;
mod output_writer;

#[proc_macro_attribute]
pub fn stable_enum(
    args: proc_macro::TokenStream,
    item: proc_macro::TokenStream
)
    -> proc_macro::TokenStream
{
    let _unused = args;

    let input = match input_reader::read_input(item.into()) {
        Ok(input) => input,
        Err(err) => {
            return err.into_compile_error().into();
        },
    };

    let output = match output_writer::generate_output(input) {
        Ok(output) => output,
        Err(err) => {
            return err.into_compile_error().into();
        },
    };

    output.into()
}
