use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{spanned::Spanned, token, Error, ItemStruct, Lifetime, Visibility};

use crate::config::Config;

pub(crate) fn generate_code(item_struct: &ItemStruct, config: &Config) -> Result<TokenStream, Error> {
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
    
    let constructor_name = config.constructor_name();
    let constructor_vis = config.constructor_visibility();
    let new_fn = quote! {
        #[inline(always)]
        #constructor_vis fn #constructor_name ( #new_args ) -> Self {
            Self { #constructor_params }
        }
    };

    let mut getters = TokenStream::new();
    for field in &fields.named {
        let field_vis = &field.vis;
        if let Visibility::Inherited = field_vis {
            continue;
        }

        let name = field.ident.as_ref().unwrap();
        let ty = &field.ty;

        let tmp_stream = if return_by_value(ty) {
            let transformed_ty = if let syn::Type::Reference(ref_ty) = ty {
                if ref_ty.lifetime.is_some() {
                    let mut cloned_ty = ref_ty.clone();
                    cloned_ty.lifetime = Some(Lifetime::new("'_", ref_ty.span()));
                    &syn::Type::Reference(cloned_ty)
                } else {
                    ty
                }
            } else {
                ty
            };

            quote! {
                #[inline(always)]
                #field_vis fn #name(&self) -> #transformed_ty {
                    self.#name
                }
            }
        } else {
            quote! {
                #[inline(always)]
                #field_vis fn #name(&self) -> &#ty {
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

    let mut cloned_struct = item_struct.clone();
    let syn::Fields::Named(fields) = &mut cloned_struct.fields else {
        return Err(Error::new(item_struct.fields.span(), "#[readonly] requires named fields."));
    };
    for field in &mut fields.named {
        field.vis = Visibility::Inherited;
    }
    Ok(quote! { #cloned_struct #impl_code })
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
