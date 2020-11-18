#![recursion_limit = "128"]

use proc_macro2::Span;
use quote::quote;
use syn::DeriveInput;

#[proc_macro_derive(Strategy, attributes(name, symbol))]
pub fn strategy_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();

    expand(&ast).into()
}

const NAME_ATTR: &str = "name";
const SYMBOL_ATTR: &str = "symbol";

fn expand(ast: &syn::DeriveInput) -> proc_macro2::TokenStream {
    let str_name = {
        match get_attribute_type_multiple(ast, NAME_ATTR) {
            Ok(res) => match res.len() {
                1 => res[0].clone(),
                _ => return err_token_stream(NAME_ATTR),
            },
            Err(err) => return err.to_compile_error(),
        }
    };

    let symbols = {
        match get_attribute_type_multiple(ast, SYMBOL_ATTR) {
            Ok(res) => res,
            Err(err) => return err.to_compile_error(),
        }
    };

    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    quote! {
        impl #impl_generics ::flashfunk_core::prelude::IntoStrategy for #name #ty_generics #where_clause {
            fn name() -> &'static str {
                #str_name
            }

            fn local_symbol() -> Vec<&'static str> {
                vec![#(#symbols),*]
            }
        }
    }
}

fn get_attribute_type_multiple(ast: &syn::DeriveInput, name: &str) -> syn::Result<Vec<String>> {
    let attr = ast
        .attrs
        .iter()
        .find_map(|a| {
            let a = a.parse_meta();
            match a {
                Ok(meta) => {
                    if meta.path().is_ident(name) {
                        Some(meta)
                    } else {
                        None
                    }
                }
                _ => None,
            }
        })
        .ok_or_else(|| {
            syn::Error::new(Span::call_site(), format!("Expect a attribute `{}`", name))
        })?;

    if let syn::Meta::List(ref list) = attr {
        Ok(list
            .nested
            .iter()
            .filter_map(|m| meta_item_to_string(m).ok())
            .collect())
    } else {
        Err(syn::Error::new_spanned(
            attr,
            format!("The correct syntax is #[{}(\"a\", \"b\", ...)]", name),
        ))
    }
}

fn meta_item_to_string(meta_item: &syn::NestedMeta) -> syn::Result<String> {
    match meta_item {
        syn::NestedMeta::Lit(syn::Lit::Str(ref s)) => Ok(s.value()),
        meta => Err(syn::Error::new_spanned(meta, "Expect string literals")),
    }
}

fn err_token_stream(attr: &str) -> proc_macro2::TokenStream {
    syn::Error::new(
        Span::call_site(),
        format!("#[{}()] takes at least 1 parameter", attr,),
    )
    .to_compile_error()
}
