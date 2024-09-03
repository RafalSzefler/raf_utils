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

/// Macro for generating stable enums, stable in the ABI sense. For example this:
/// 
/// ```rust
/// use raf_stable_enum::stable_enum;
/// 
/// #[stable_enum]
/// pub enum MyEnum {
///     UNKNOWN = 1,
///     FOO(i32) = 2,
///     BAZ(String) = 3,
/// }
/// ```
/// 
/// will generate (more or less) this code:
/// 
/// ```rust
/// #![allow(non_camel_case_types)]
/// #![allow(non_snake_case)]
///
/// #[repr(u32)]
/// #[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
/// pub enum MyEnumKey {
///     UNKNOWN = 1u32,
///     FOO = 2u32,
///     BAZ = 3u32,
/// }
///
/// mod _raf_stable_enum_tmp {
///     use super::*;
///
///     #[repr(C)]
///     pub(super) struct _Tuple_2<T> {
///         pub key: MyEnumKey,
///         pub val: T,
///     }
///
///     #[repr(C)]
///     pub(super) union _Union_2 {
///         pub _raw_key: MyEnumKey,
///         pub FOO: ::core::mem::ManuallyDrop<_Tuple_2<i32>>,
///         pub BAZ: ::core::mem::ManuallyDrop<_Tuple_2<String>>,
///     }
/// }
///
/// #[repr(C)]
/// pub struct MyEnum {
///     internal: _raf_stable_enum_tmp::_Union_2,
/// }
///
/// impl MyEnum {
///     #[inline(always)]
///     pub fn key(&self) -> MyEnumKey {
///         unsafe { self.internal._raw_key }
///     }
///
///     #[inline(always)]
///     pub unsafe fn FOO(&self) -> &i32 {
///         &self.internal.FOO.val
///     }
///
///     #[inline(always)]
///     pub unsafe fn BAZ(&self) -> &String {
///         &self.internal.BAZ.val
///     }
/// }
///
/// impl Drop for MyEnum {
///     fn drop(&mut self) {
///         unsafe {
///             match self.internal._raw_key {
///                 MyEnumKey::FOO => {
///                     let _ = ::core::mem::ManuallyDrop::take(&mut self.internal.FOO);
///                 }
///                 MyEnumKey::BAZ => {
///                     let _ = ::core::mem::ManuallyDrop::take(&mut self.internal.BAZ);
///                 }
///                 _ => {}
///             }
///         }
///     }
/// }
///
/// pub struct MyEnumBuilder;
///
/// impl MyEnumBuilder {
///     #[inline(always)]
///     pub fn create_UNKNOWN() -> MyEnum {
///         let internal = _raf_stable_enum_tmp::_Union_2 {
///             _raw_key: MyEnumKey::UNKNOWN,
///         };
///         MyEnum { internal: internal }
///     }
///
///     #[inline(always)]
///     pub fn create_FOO(value: i32) -> MyEnum {
///         let tmp = _raf_stable_enum_tmp::_Tuple_2 {
///             key: MyEnumKey::FOO,
///             val: value,
///         };
///         let internal = _raf_stable_enum_tmp::_Union_2 {
///             FOO: ::core::mem::ManuallyDrop::new(tmp),
///         };
///         MyEnum { internal: internal }
///     }
///
///     #[inline(always)]
///     pub fn create_BAZ(value: String) -> MyEnum {
///         let tmp = _raf_stable_enum_tmp::_Tuple_2 {
///             key: MyEnumKey::BAZ,
///             val: value,
///         };
///         let internal = _raf_stable_enum_tmp::_Union_2 {
///             BAZ: ::core::mem::ManuallyDrop::new(tmp),
///         };
///         MyEnum { internal: internal }
///     }
/// }
/// ```
/// 
/// which basically ensures enums ABI, by converting all variants into a C union with overlapping layouts.
#[proc_macro_attribute]
pub fn stable_enum(
    _args: proc_macro::TokenStream,
    item: proc_macro::TokenStream
)
    -> proc_macro::TokenStream
{
    let input = match input_reader::read_input(item.into()) {
        Ok(input) => input,
        Err(err) => {
            return err.into_compile_error().into();
        },
    };

    output_writer::generate_output(&input).into()
}

