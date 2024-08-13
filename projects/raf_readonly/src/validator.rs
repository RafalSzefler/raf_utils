use syn::{spanned::Spanned, Error, ItemStruct};

use crate::config::Config;

pub(crate) fn validate_item_struct(item_struct: &ItemStruct, config: &Config) -> Option<Error> {
    let syn::Fields::Named(fields) = &item_struct.fields else {
        return Some(Error::new(item_struct.fields.span(), "#[readonly] requires named fields."));
    };

    for field in &fields.named {
        if let Some(name) = &field.ident {
            if name == config.constructor_name() {
                let msg = format!("#[readonly] name conflict with constructor name: [{}].", config.constructor_name());
                return Some(Error::new(name.span(), msg));
            }
            if config.with_release() && name == config.release_name() {
                let msg = format!("#[readonly] name conflict with release name: [{}].", config.release_name());
                return Some(Error::new(name.span(), msg));
            }
        } else {
            return Some(Error::new(field.span(), "#[readonly] requires named fields."));            
        }

        match &field.ty {
            syn::Type::Ptr(real_ty) => {
                if let Some(mut_tk) = real_ty.mutability {
                    return Some(Error::new(mut_tk.span(), "#[readonly] does not allow mutable pointers. For internal mutability use Cells instead."));
                }
            }
            syn::Type::Reference(real_ty) => {
                if let Some(mut_tk) = real_ty.mutability {
                    return Some(Error::new(mut_tk.span(), "#[readonly] does not allow mutable references. For internal mutability use Cells instead."));
                }
            },
            _ => { }
        }
    }

    None
}
