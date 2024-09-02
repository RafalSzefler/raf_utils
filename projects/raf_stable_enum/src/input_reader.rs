use std::collections::HashSet;

use proc_macro2::TokenStream;
use raf_readonly::readonly;
use syn::{parse2, spanned::Spanned, Error, Ident, ItemEnum, Type, Visibility};

#[readonly]
pub(crate) struct ParsedEnumVariant {
    pub name: Ident,
    pub associated_type: Option<Type>,
    pub value: u32,
}

#[readonly]
pub(crate) struct ParsedEnum {
    pub visibility: Visibility,
    pub name: Ident,
    pub variants: Vec<ParsedEnumVariant>,
}

pub(crate) fn read_input(tokens: TokenStream) -> Result<ParsedEnum, Error>
{
    let enum_item = parse2::<ItemEnum>(tokens)?;

    let mut errors = Vec::<Error>::new();

    macro_rules! return_error {
        () => {
            {
                return Err(into_error(errors));
            }
        };
    }
    
    if !enum_item.attrs.is_empty() {
        macro_rules! make_error {
            ( $attr: expr ) => {
                {
                    let span = { $attr }.bracket_token.span.span();
                    Error::new(span, "[stable_enum] enums cannot have custom attributes.")
                }
            };
        }
        let mut iter = enum_item.attrs.iter();
        let first = iter.next().unwrap();
        errors.push(make_error!(first));
        for attr in iter {
            errors.push(make_error!(attr));
        }
    }

    if enum_item.variants.is_empty() {
        errors.push(Error::new(enum_item.ident.span(), "[stable_enum] enums have to have at least one variant, cannot be empty."));
        return_error!();
    }

    let mut variants = Vec::with_capacity(enum_item.variants.len());

    let mut seen_values = HashSet::with_capacity(enum_item.variants.len());

    for variant in &enum_item.variants {
        if variant.fields.len() > 1 {
            errors.push(Error::new(variant.fields.span(), "[stable_enum] enums variants can have a single associated type."));
        }

        let name = variant.ident.to_string();
        if name.chars().any(|ch| ch.is_lowercase()) {
            errors.push(Error::new(variant.ident.span(), "[stable_enum] enums variant names have to be upper case."));
        }

        if let Some(discriminant) = &variant.discriminant {
            match &discriminant.1 {
                syn::Expr::Lit(val) => {
                    match &val.lit {
                        syn::Lit::Int(val) => {
                            let value = match val.base10_parse::<u32>() {
                                Ok(value) => value,
                                Err(_) => {
                                    errors.push(Error::new(discriminant.1.span(), "[stable_enum] enums have to have u32 const numeric values."));
                                    continue;
                                },
                            };

                            if !seen_values.insert(value) {
                                errors.push(Error::new(discriminant.1.span(), "[stable_enum] value already present in the enum."));
                            }

                            let associated_type = if variant.fields.is_empty() {
                                None
                            } else {
                                let field = variant.fields.iter().next().unwrap();
                                Some(field.ty.clone())
                            };

                            variants.push(ParsedEnumVariant::new(
                                variant.ident.clone(),
                                associated_type,
                                value))
                        },
                        _ => {
                            errors.push(Error::new(discriminant.1.span(), "[stable_enum] enums have to have const numeric values."));
                        },
                    }
                },
                _ => {
                    errors.push(Error::new(discriminant.1.span(), "[stable_enum] enums have to have const numeric values."));
                },
            }
        } else {
            errors.push(Error::new(variant.span(), "[stable_enum] enums have to have const numeric values."));
        }
    }

    if !errors.is_empty() {
        return_error!();
    }

    Ok(ParsedEnum::new(enum_item.vis.clone(), enum_item.ident.clone(), variants))
}

fn into_error(errors: Vec<Error>) -> Error {
    let mut iter = errors.into_iter();
    let mut result = iter.next().unwrap();
    for err in iter {
        result.extend(err);
    }
    result
}
