#![warn(clippy::all, clippy::pedantic)]
#![allow(
    clippy::needless_return,
    clippy::redundant_field_names,
    clippy::unreadable_literal,
    clippy::inline_always,
    clippy::must_use_candidate,
    clippy::module_name_repetitions,
)]

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    parse_macro_input, spanned::Spanned, token, Error, ItemStruct, Lifetime, Visibility
};


#[proc_macro_attribute]
pub fn readonly(
    args: proc_macro::TokenStream,
    item: proc_macro::TokenStream
) -> proc_macro::TokenStream
{
    if let Some(tk) = args.into_iter().next() {
        let error = Error::new(tk.span().into(), "#[readonly] attribute does not accept arguments.");
        return error.to_compile_error().into();
    }

    let item_struct = parse_macro_input!(item as ItemStruct);
    if let Some(err) = validate_item_struct(&item_struct) {
        return err.to_compile_error().into();
    }

    match generate_code(&item_struct) {
        Ok(stream) => stream.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

fn validate_item_struct(item_struct: &ItemStruct) -> Option<Error> {
    let syn::Fields::Named(fields) = &item_struct.fields else {
        return Some(Error::new(item_struct.fields.span(), "#[readonly] requires named fields."));
    };

    for field in &fields.named {
        if let Visibility::Inherited = field.vis { } else {
            return Some(Error::new(field.span(), "#[readonly] requires private fields."));
        }

        if let Some(name) = &field.ident {
            if name == "new" {
                return Some(Error::new(name.span(), "#[readonly] does not allow field name to be \"new\"."));
            }
        } else {
            return Some(Error::new(field.span(), "#[readonly] requires named fields."));            
        }
    }

    None
}

fn generate_code(item_struct: &ItemStruct) -> Result<proc_macro2::TokenStream, Error> {
    let vis = &item_struct.vis;
    let name = &item_struct.ident;
    let generics = &item_struct.generics;

    let syn::Fields::Named(fields) = &item_struct.fields else {
        return Err(Error::new(item_struct.fields.span(), "#[readonly] requires named fields."));
    };

    let mut new_args = TokenStream::new();
    let mut constructor_params = TokenStream::new();

    let mut field_iter = fields.named.iter();
    if let Some(first_field) = field_iter.next() {
        process_field_as_arg(first_field, &mut new_args);
        process_field_for_constructor(first_field, &mut constructor_params);
        let comma = token::Comma::default();
        for field in field_iter {
            comma.to_tokens(&mut new_args);
            process_field_as_arg(field, &mut new_args);
            comma.to_tokens(&mut constructor_params);
            process_field_for_constructor(field, &mut constructor_params);
        }
    }
    

    let new_fn = quote! {
        #[inline(always)]
        #vis fn new( #new_args ) -> Self {
            Self { #constructor_params }
        }
    };

    let mut getters = TokenStream::new();
    for field in &fields.named {
        let name = field.ident.as_ref().unwrap();
        let ty = &field.ty;

        let tmp_stream = if return_by_value(ty) {
            let transformed_ty = if let syn::Type::Reference(ref_ty) = ty {
                let mut cloned_ty = ref_ty.clone();
                if cloned_ty.mutability.is_some() {
                    return Err(Error::new(ref_ty.span(), "#[readonly] does not allow mutable references. For internal mutability use Cells instead."));
                }

                if cloned_ty.lifetime.is_some() {
                    cloned_ty.lifetime = Some(Lifetime::new("'_", ref_ty.span()));
                }
                &syn::Type::Reference(cloned_ty)
            } else {
                ty
            };

            quote! {
                #[inline(always)]
                #vis fn #name(&self) -> #transformed_ty {
                    self.#name
                }
            }
        } else {
            quote! {
                #[inline(always)]
                #vis fn #name(&self) -> &#ty {
                    &self.#name
                }
            }
        };

        tmp_stream.to_tokens(&mut getters);
    }

    let where_clause = &item_struct.generics.where_clause;

    let impl_code = quote! {
        impl #generics #name #generics #where_clause {
            #new_fn
            #getters
        }
    };
    Ok(quote! { #item_struct #impl_code })
}


fn return_by_value(ty: &syn::Type) -> bool {
    if let syn::Type::Path(path) = ty {
        let segments = &path.path.segments;
        if segments.len() == 1 {
            let first_seg = segments.first().unwrap();
            let name = first_seg.ident.to_string();
            match name.as_str() {
                "bool" | "i8" | "u8" | "i16" | "u16" | "i32" | "u32" | "i64" | "u64" | "isize" | "usize" => return true,
                _ => { }
            }
        }
    }

    matches!(ty, syn::Type::Ptr(_) | syn::Type::Reference(_))
}

fn process_field_as_arg(field: &syn::Field, tokens: &mut TokenStream) {
    let name = field.ident.as_ref().unwrap();
    let ty = &field.ty;
    quote! { #name: #ty }.to_tokens(tokens);
}

fn process_field_for_constructor(field: &syn::Field, tokens: &mut TokenStream) {
    let name = field.ident.as_ref().unwrap();
    quote! { #name: #name }.to_tokens(tokens);
}
