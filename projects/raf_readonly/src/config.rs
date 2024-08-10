use std::collections::HashMap;

use syn::{parse::{Parse, ParseStream}, Ident, Token, Visibility};

pub(crate) struct Config {
    constructor_name: Ident,
    constructor_visibility: Visibility,
}

impl Config {
    #[inline(always)]
    pub fn constructor_name(&self) -> &Ident {
        &self.constructor_name
    }

    #[inline(always)]
    pub fn constructor_visibility(&self) -> &Visibility {
        &self.constructor_visibility
    }
}

const CONSTRUCTOR_NAME: &str = "ctr_name";
const DEFAULT_NAME: &str = "new";
const CONSTRUCTOR_VISIBILITY: &str = "ctr_vis";


impl Parse for Config {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let input_span = input.span();
        let kvs = input.parse_terminated(KeyValue::parse, Token![,])?;
        let mut map = HashMap::with_capacity(kvs.len());
        for kv in kvs {
            let span = kv.span;
            if map.insert(kv.key.clone(), kv).is_some() {
                return Err(syn::Error::new(span, "#[readonly] Value for this key already set."));
            }
        }

        let constructor_name = if let Some(value) = map.remove(CONSTRUCTOR_NAME) {
            if let Value::Ident(ident) = value.value {
                ident
            } else {
                return Err(syn::Error::new(value.span, "#[readonly] ctr_name value has to be an identifier."));
            }
        } else {
            Ident::new(DEFAULT_NAME, input_span)
        };

        let constructor_visibility = if let Some(value) = map.remove(CONSTRUCTOR_VISIBILITY) {
            if let Value::Vis(vis) = value.value {
                vis
            } else {
                return Err(syn::Error::new(value.span, "#[readonly] ctr_vis value has to be visibility attribute."));
            }
        } else {
            Visibility::Public(syn::token::Pub::default())
        };

        if let Some(kv) = map.into_iter().next() {
            return Err(syn::Error::new(kv.1.span, "#[readonly] Unrecognized option."));
        }

        Ok(Config {
            constructor_name: constructor_name,
            constructor_visibility: constructor_visibility,
        })
    }
}

enum Value {
    Vis(Visibility),
    Ident(Ident),
}

impl Parse for Value {
    fn parse(input: ParseStream) -> syn::Result<Self> {        
        if input.peek(Token![pub]) {
            let vis = input.parse::<Visibility>()?;
            return Ok(Value::Vis(vis));
        }

        if input.peek(Token![priv]) {
            let _ = input.parse::<Token![priv]>();
            return Ok(Value::Vis(Visibility::Inherited));
        }

        if input.peek(Ident) {
            let ident = input.parse::<Ident>()?;
            return Ok(Value::Ident(ident));
        }

        syn::Result::Err(input.error("#[readonly] Invalid value. Expected either identifier or visibility attribute."))
    }
}

struct KeyValue {
    pub span: proc_macro2::Span,
    pub key: String,
    pub value: Value,
}

fn is_valid_repr(text: &str, span: proc_macro2::Span) -> Result<(), syn::Error> {
    if !text.is_ascii() {
        return Err(syn::Error::new(span, "#[readonly] Value has to contain ascii characters only."));
    }

    if text.len() > 255 {
        return Err(syn::Error::new(span, "#[readonly] Value too long. Expecting at most 255 characters."));
    }

    let chars: Vec<char> = text.chars().collect();

    if chars.iter().any(|chr| { chr.is_whitespace() }) {
        return Err(syn::Error::new(span, "#[readonly] Value cannot contain whitespace."));
    }

    Ok(())
}

impl Parse for KeyValue {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let key_token = input.parse::<Ident>()?;
        let key = key_token.to_string();
        is_valid_repr(&key, key_token.span())?;
        let _ = input.parse::<syn::token::Eq>()?;
        let value = input.parse::<Value>()?;
        syn::Result::Ok(KeyValue {
            span: key_token.span(),
            key: key,
            value: value,
        })
    }
}
