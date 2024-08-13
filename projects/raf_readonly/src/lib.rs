//! proc-macro project that holds [`macro@readonly`] attribute for generating
//! readonly structs out of standard Rust structs.
#![warn(clippy::all, clippy::pedantic)]
#![allow(
    clippy::needless_return,
    clippy::redundant_field_names,
    clippy::unreadable_literal,
    clippy::inline_always,
    clippy::must_use_candidate,
    clippy::module_name_repetitions,
)]

mod config;
mod validator;
mod code_generator;

use code_generator::generate_code;
use config::Config;
use syn::{parse_macro_input, ItemStruct};
use validator::validate_item_struct;


/// Generates readonly only struct out of decorated struct. All fields
/// get converted to private fields. It generates a constructor, and a getter
/// per non-private field.
/// 
/// # Options:
/// 
/// The attribute accepts the following options:
/// * `ctr_name = new` which sets the name of constructor to `new`. The value
///   has to be a valid, non-keyword identifier. By default this is `new`.
/// * `ctr_vis = pub` which sets visibility of the constructor. Use `priv` for
///   private/inherited visibility. By default this is `pub`.
/// * `with_release = false` generates a copy of decorated struct, with name
///   prefixed with `Release` identifier. The copied struct does not have any
///   functionalities, but serves as way of releasing the [`macro@readonly`] marked
///   struct, so that we can move out its pieces. This additionally generates
///   `fn release(self) -> Release*` function on the original struct, with the
///   same visibility as the struct itself. By default this option is
///   set to `false`.
/// * `release_name = release` the name of generated release function.
/// 
/// # Example:
/// 
/// The attribute applied like here:
/// 
/// ```rust
/// mod tests {
///     use std::time::Instant;
///     use raf_readonly::readonly;
///
///     struct User;
///
///     #[readonly(ctr_name=constructor, ctr_vis=pub(super))]
///     struct Account
///     {
///         pub name: String,
///         pub owner: User,
///         pub id: u64,
///         pub created_at: Instant,
///         balance: i64,
///     }
/// }
/// ```
/// 
/// will generate the following code:
/// 
/// ```rust
/// mod tests {
///     use std::time::Instant;
///
///     struct User;
///
///     struct Account {
///         name: String,
///         owner: User,
///         id: u64,
///         created_at: Instant,
///         balance: i64,
///     }
///
///     impl Account {
///         #[inline(always)]
///         pub(super) fn constructor(
///             name: String,
///             owner: User,
///             id: u64,
///             created_at: Instant,
///             balance: i64,
///         ) -> Self {
///             Self {
///                 name: name,
///                 owner: owner,
///                 id: id,
///                 created_at: created_at,
///                 balance: balance,
///             }
///         }
///
///         #[inline(always)]
///         pub fn name(&self) -> &String {
///             &self.name
///         }
///
///         #[inline(always)]
///         pub fn owner(&self) -> &User {
///             &self.owner
///         }
///
///         #[inline(always)]
///         pub fn id(&self) -> u64 {
///             self.id
///         }
///
///         #[inline(always)]
///         pub fn created_at(&self) -> &Instant {
///             &self.created_at
///         }
///     }
/// }
/// ```
/// 
/// # Notes:
/// * Some getters return by value, not by reference. At the moment those are
///   hardcoded primitive types
/// * All generated getters are marked with `#[inline(always)]`. This is not
///   configurable at the moment.
#[proc_macro_attribute]
pub fn readonly(
    args: proc_macro::TokenStream,
    item: proc_macro::TokenStream
) -> proc_macro::TokenStream
{
    let config = parse_macro_input!(args as Config);

    let item_struct = parse_macro_input!(item as ItemStruct);
    if let Some(err) = validate_item_struct(&item_struct, &config) {
        return err.to_compile_error().into();
    }

    match generate_code(&item_struct, &config) {
        Ok(stream) => stream.into(),
        Err(err) => err.to_compile_error().into(),
    }
}
