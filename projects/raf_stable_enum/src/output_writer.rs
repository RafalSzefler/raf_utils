use std::sync::atomic::{AtomicU32, Ordering};

use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use syn::Ident;

use crate::input_reader::ParsedEnum;

static COUNTER: AtomicU32 = AtomicU32::new(0);
#[inline(always)]
fn get_next_id() -> u32 {
    COUNTER.fetch_add(1, Ordering::SeqCst)
}

const PRIV_MOD: &str = "_raf_stable_enum_tmp";

#[allow(clippy::too_many_lines)]
pub(crate) fn generate_output(input: &ParsedEnum) -> TokenStream
{
    let has_variant_types = input.variants().iter().any(|v| v.associated_type().is_some());
    let vis = input.visibility();
    let input_name = input.name();
    let input_name_str = input_name.to_string();

    let key_name = if has_variant_types {
        let key_name_str = format!("{input_name_str}Key");
        &Ident::new(&key_name_str, Span::call_site())
    } else {
        input_name
    };

    let values_stream = {
        let mut stream = TokenStream::new();
        for variant in input.variants() {
            let variant_name = variant.name();
            let value = variant.value();
            quote! {
                #variant_name = #value,
            }.to_tokens(&mut stream);
        }
        stream
    };

    let union_stream = if has_variant_types {
        let priv_mod = Ident::new(PRIV_MOD, Span::call_site());
        let tmp_id = get_next_id();
        let tmp_name = Ident::new(&format!("_Tuple_{tmp_id}"), Span::call_site());

        let union_name = Ident::new(
            &format!("_Union_{tmp_id}"),
            Span::call_site());
        let builder_name = Ident::new(
            &format!("{input_name_str}Builder"),
            Span::call_site());

        let mut union_fields = TokenStream::new();
        let mut union_impl = TokenStream::new();
        let mut drop_body = TokenStream::new();
        let mut builder_impl = TokenStream::new();

        let mut missing_drop = false;

        for variant in input.variants() {
            let name = variant.name();
            let create_name = Ident::new(&format!("create_{name}"), Span::call_site());

            if let Some(associated_type) = variant.associated_type() {
                quote! {
                    pub #name: ::core::mem::ManuallyDrop<#tmp_name<#associated_type>>,
                }.to_tokens(&mut union_fields);

                let doc_ref= format!(" It is safe to use this method only if key is equal to [`{key_name}::{name}`]. It is up to caller to ensure that.");

                quote! {
                    #[doc=r" Retrieves value associated with the underlying key."]
                    #[doc=r" "]
                    #[doc=r" # Safety"]
                    #[doc= #doc_ref]
                    #[allow(non_snake_case)]
                    #[inline(always)]
                    pub unsafe fn #name(&self) -> &#associated_type {
                        &self.internal.#name.val
                    }
                }.to_tokens(&mut union_impl);

                quote! {
                    #key_name::#name => {
                        let _ = ::core::mem::ManuallyDrop::take(&mut self.internal.#name);
                    },
                }.to_tokens(&mut drop_body);

                quote! {
                    #[allow(non_snake_case)]
                    #[inline(always)]
                    pub fn #create_name(value: #associated_type) -> #input_name {
                        let tmp = #priv_mod::#tmp_name {
                            key: #key_name::#name,
                            val: value,
                        };
                        let internal = #priv_mod::#union_name {
                            #name: ::core::mem::ManuallyDrop::new(tmp),
                        };
                        #input_name { internal: internal }
                    }
                }.to_tokens(&mut builder_impl);
            } else {
                quote! {
                    #[allow(non_snake_case)]
                    #[inline(always)]
                    pub fn #create_name() -> #input_name {
                        let internal = #priv_mod::#union_name { _raw_key: #key_name::#name };
                        #input_name { internal: internal }
                    }
                }.to_tokens(&mut builder_impl);

                missing_drop = true;
            }
        }

        if missing_drop {
            quote! {
                _ => { },
            }.to_tokens(&mut drop_body);
        }


        quote! {
            #[allow(non_camel_case_types)]
            #[allow(non_snake_case)]
            mod #priv_mod {
                use super::*;

                #[repr(C)]
                pub(super) struct #tmp_name <T> {
                    pub key: #key_name,
                    pub val: T
                }

                #[repr(C)]
                pub(super) union #union_name {
                    pub _raw_key: #key_name,
                    #union_fields
                }
            }

            #[repr(C)]
            #vis struct #input_name {
                internal: #priv_mod::#union_name,
            }

            impl #input_name {
                /// Retrieves the underlying key.
                #[inline(always)]
                pub fn key(&self) -> #key_name {
                    unsafe { self.internal._raw_key }
                }

                #union_impl
            }

            impl Drop for #input_name {
                fn drop(&mut self) {
                    unsafe {
                        match self.internal._raw_key {
                            #drop_body
                        }
                    }
                }
            }

            #vis struct #builder_name;

            impl #builder_name {
                #builder_impl
            }
        }
    } else {
        TokenStream::new()
    };


    quote! {
        #[repr(u32)]
        #[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
        #vis enum #key_name {
            #values_stream
        }

        #union_stream
    }.to_token_stream()
}
