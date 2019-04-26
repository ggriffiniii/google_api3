extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use serde_derive_internals as serdei;
use std::error::Error;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(FieldSelector)]
pub fn derive_field_selector(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    expand_derive_field_selector(&input).unwrap().into()
}

fn expand_derive_field_selector(input: &DeriveInput) -> Result<TokenStream2, Box<Error>> {
    let ctx = serdei::Ctxt::new();
    let cont = serdei::ast::Container::from_ast(&ctx, &input, serdei::Derive::Deserialize);
    ctx.check()?;
    let field_output: Vec<proc_macro2::TokenStream> = match cont.data {
        serdei::ast::Data::Struct(serdei::ast::Style::Struct, fields) => {
            fields.iter().map(selector_for_field).collect()
        }
        _ => return Err("Only able to derive FieldSelector for plain Struct".into()),
    };

    let ident = cont.ident;
    let (impl_generics, ty_generics, where_clause) = cont.generics.split_for_impl();
    Ok(quote! {
        impl #impl_generics google_api3::FieldSelector for #ident #ty_generics #where_clause {
            fn field_selector_with_ident(ident: &str, selector: &mut String) {
                match selector.chars().rev().nth(0) {
                    Some(',') | None => {},
                    _ => selector.push_str(","),
                }
                #(#field_output)*
            }
        }
    })
}

fn selector_for_field<'a>(field: &serdei::ast::Field<'a>) -> TokenStream2 {
    let field_name = field.attrs.name().deserialize_name();
    let typ = field.ty;
    if field.attrs.flatten() {
        quote! {
            <#typ>::field_selector_with_ident(ident, selector);
        }
    } else {
        quote! {
            <#typ>::field_selector_with_ident(&{
                if ident.is_empty() {
                    #field_name.to_owned()
                } else {
                    let mut ident = ident.to_owned();
                    ident.push_str("/");
                    ident.push_str(#field_name);
                    ident
                }},
                selector);
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
