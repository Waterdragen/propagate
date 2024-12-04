use std::collections::HashMap;
use syn::{Fields, Variant};
use quote::quote;
use proc_macro2::{Ident, Literal, Span, TokenStream};

const GOOD_ATTR_NAME: &str = "good";
const BAD_ATTR_NAME: &str = "bad";

pub fn has_attribute(variant: &Variant, ident: &str) -> bool {
    variant.attrs.iter().any(|attr| attr.path().is_ident(ident))
}

pub fn has_good_attribute(variant: &Variant) -> bool {
    has_attribute(variant, GOOD_ATTR_NAME)
}

pub fn has_bad_attribute(variant: &Variant) -> bool {
    has_attribute(variant, BAD_ATTR_NAME)
}

pub fn ensure_unit_or_tuple_struct(variant: &Variant) -> bool {
    if matches!(variant.fields, Fields::Named(..)) {
        panic!("Named struct cannot have this attribute");
    }
    true
}

pub fn get_tuple_field_type(fields: &Fields, borrow: TokenStream) -> TokenStream {
    match fields {
        Fields::Unit => quote! { #borrow () },
        Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
            let ty = &fields.unnamed[0].ty;
            quote! { #borrow #ty }
        }
        Fields::Unnamed(fields) => {
            let types = fields.unnamed.iter().map(|field| &field.ty);
            quote! { (#(#borrow #types),*) }
        }
        Fields::Named(_) => unreachable!()
    }
}

pub fn get_any_field_input_and_output(fields: &Fields) -> (TokenStream, TokenStream) {
    match fields {
        Fields::Unit => (quote! {}, quote! { () }),
        Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
            (quote! { (v) }, quote! { v })
        },
        Fields::Unnamed(fields) => {
            let len = fields.unnamed.len();
            let types = (0..len).map(|i| {
                let s = String::from("v") + &i.to_string();
                Ident::new(&s, Span::call_site())
            });
            let tuple = quote! { (#(#types),*) };
            (tuple.clone(), tuple)
        },
        Fields::Named(_) => (quote! { {..} }, quote! { () }),
    }
}

pub fn group_variant_by_type(variants: &[Variant]) -> HashMap<Fields, Vec<Variant>> {
    let mut grouped_variants: HashMap<Fields, Vec<Variant>> = HashMap::new();
    for variant in variants.iter().cloned() {
        let fields = variant.fields.clone();
        grouped_variants.entry(fields).or_default().push(variant);
    }
    grouped_variants
}

pub fn get_index_map(enum_name: &Ident, variants: &[Variant]) -> Vec<TokenStream> {
    variants
        .iter()
        .enumerate()
        .map(|(index, variant)| {
            let variant_name = &variant.ident;
            let (input, _) = get_any_field_input_and_output(&variant.fields);
            let index = Literal::usize_unsuffixed(index);

            quote! {#enum_name::#variant_name #input => #index,}
        })
        .collect()
}
