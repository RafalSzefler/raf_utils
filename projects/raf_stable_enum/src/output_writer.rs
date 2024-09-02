use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{Error, Ident};

use crate::input_reader::ParsedEnum;


pub(crate) fn generate_output(input: ParsedEnum) -> Result<TokenStream, Error>
{
    let has_variant_types = input.variants().iter().any(|v| v.associated_type().is_some());
    let vis = input.visibility();
    let input_name = input.name();
    let name_str = input_name.to_string();

    let key_name = if has_variant_types {
        let key_name_str = format!("{0}Key", name_str);
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

    let union_stream = if !has_variant_types {
        TokenStream::new()
    } else {
        let union_name = Ident::new(
            &format!("_{0}Union", name_str),
            Span::call_site());
        let builder_name = Ident::new(
            &format!("{0}Builder", name_str),
            Span::call_site());

        let mut union_fields = TokenStream::new();
        let mut union_impl = TokenStream::new();
        let mut match_generics = TokenStream::new();
        let mut match_where = TokenStream::new();
        let mut match_args = TokenStream::new();
        let mut match_body = TokenStream::new();
        let mut drop_body = TokenStream::new();
        let mut builder_impl = TokenStream::new();

        let mut first_variant = true;
        let mut missing_drop = false;

        for variant in input.variants() {
            let name = variant.name();
            let generic_name = Ident::new(&format!("F{0}", name), Span::call_site());    
            let generic_arg = Ident::new(&format!("f{0}", name), Span::call_site());
            let create_name = Ident::new(&format!("create_{0}", name), Span::call_site());
            let mut fnonce = TokenStream::new();
            let mut call_stream = TokenStream::new();

            if let Some(associated_type) = variant.associated_type() {
                quote! {
                    #name: ::core::mem::ManuallyDrop<#associated_type>,
                }.to_tokens(&mut union_fields);

                quote! {
                    #[allow(non_snake_case)]
                    #[inline(always)]
                    pub unsafe fn #name(&self) -> &#associated_type {
                        &self.internal.#name
                    }
                }.to_tokens(&mut union_impl);

                quote! {
                    FnOnce( &#associated_type )
                }.to_tokens(&mut fnonce);

                quote! {
                    #generic_arg(unsafe { &self.internal.#name });
                }.to_tokens(&mut call_stream);

                quote! {
                    #key_name::#name => {
                        let _ = ::core::mem::ManuallyDrop::take(&mut self.internal.#name);
                    },
                }.to_tokens(&mut drop_body);

                quote! {
                    #[allow(non_snake_case)]
                    #[inline(always)]
                    pub fn #create_name(value: #associated_type) -> #input_name {
                        let internal = #union_name {
                            #name: ::core::mem::ManuallyDrop::new(value),
                        };
                        #input_name { key: #key_name::#name, internal: internal }
                    }
                }.to_tokens(&mut builder_impl);
            } else {
                quote! {
                    FnOnce()
                }.to_tokens(&mut fnonce);


                quote! {
                    #generic_arg();
                }.to_tokens(&mut call_stream);

                quote! {
                    #[allow(non_snake_case)]
                    #[inline(always)]
                    pub fn #create_name() -> #input_name {
                        let internal = #union_name { _empty: () };
                        #input_name { key: #key_name::#name, internal: internal }
                    }
                }.to_tokens(&mut builder_impl);

                missing_drop = true;
            }

            quote! {
                #key_name::#name => {
                    #call_stream
                },
            }.to_tokens(&mut match_body);

            if first_variant {
                quote! { #generic_name: #fnonce }.to_tokens(&mut match_where);
                generic_name.to_tokens(&mut match_generics);
                quote! { #generic_arg: #generic_name }.to_tokens(&mut match_args);
                first_variant = false;
            } else {
                quote! { , #generic_name }.to_tokens(&mut match_generics);
                quote! { , #generic_name: #fnonce }.to_tokens(&mut match_where);
                quote! { , #generic_arg: #generic_name }.to_tokens(&mut match_args);
            }
        }

        if missing_drop {
            quote! {
                _ => { },
            }.to_tokens(&mut drop_body);
        }

        quote! {
            #[allow(non_snake_case)]
            #[repr(C)]
            union #union_name {
                _empty: (),
                #union_fields
            }

            #[repr(C)]
            #vis struct #input_name {
                key: #key_name,
                internal: #union_name,
            }

            impl #input_name {
                #[inline(always)]
                pub fn key(&self) -> #key_name {
                    self.key
                }

                #union_impl

                #[allow(non_snake_case)]
                #[inline(always)]
                pub fn match_it< #match_generics >( &self, #match_args )
                    where #match_where
                {
                    match self.key {
                        #match_body
                    }
                } 
            }

            impl Drop for #input_name {
                fn drop(&mut self) {
                    unsafe {
                        match self.key {
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
    };


    let final_stream = quote! {
        #[repr(u32)]
        #[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
        #vis enum #key_name {
            #values_stream
        }

        #union_stream
    }.to_token_stream();

    Ok(final_stream)
}
