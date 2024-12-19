use alloc::vec::Vec;
use alloc::string::{String, ToString};
use hashbrown::{HashMap, HashSet};
use proc_macro2::{Ident, Literal, Span, TokenStream, TokenStream as TokenStream2};
use quote::quote;
use syn::{Field, Fields, Type, Variant};

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
    if matches!(variant.fields, Fields::Named(_)) {
        panic!("Named struct cannot have this attribute");
    }
    true
}

pub fn get_tuple_field_type(fields: &Fields, borrow: &TokenStream) -> TokenStream {
    match fields {
        Fields::Unit => quote! { () },
        Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
            let ty = &fields.unnamed[0].ty;
            quote! { #borrow #ty }
        },
        Fields::Unnamed(fields) => {
            let types = fields.unnamed.iter().map(|field| &field.ty);
            quote! { (#(#borrow #types),*) }
        },
        Fields::Named(_) => unreachable!(),
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
        // For matching indexes only
        Fields::Named(_) => (quote! { {..} }, quote! { () }),
    }
}

pub fn group_variant_ref_by_type<'a>(variant: &'a [&Variant]) -> HashMap<&'a Fields, Vec<&'a Variant>> {
    let mut grouped_variants: HashMap<&Fields, Vec<&Variant>> = HashMap::new();
    for variant in variant.iter() {
        let fields = &variant.fields;
        grouped_variants.entry(fields).or_default().push(*variant);
    }
    grouped_variants
}

pub fn get_index_matcher(enum_name: &Ident, variants: &[Variant]) -> Vec<TokenStream> {
    variants
        .iter()
        .enumerate()
        .map(|(index, variant)| {
            let variant_name = &variant.ident;
            let index = Literal::usize_unsuffixed(index);
            quote! {#enum_name::#variant_name {..} => #index,}
        })
        .collect()
}

pub fn validate_grouped_variants<'a, I>(variants: I) -> Result<(), (Vec<&'a Type>, Vec<&'a Type>)>
    where I: Iterator<Item = &'a &'a Fields> {
    let mut tuple_like_type_set = HashSet::new();
    for fields in variants {
        let fields = if let Fields::Unnamed(fields) = fields {fields} else {continue};
        let fields: Vec<&Field> = fields.unnamed.iter().collect();
        let types: Vec<&Type>;
        if fields.len() == 1 {
            let tuple_ty = if let Type::Tuple(tuple_ty) = &fields[0].ty {tuple_ty} else {continue};
            types = tuple_ty.elems.iter().collect::<Vec<_>>();
        } else {
            types = fields.iter().map(|field| &field.ty).collect::<Vec<_>>();
        }
        match tuple_like_type_set.take(&types) {
            None => { tuple_like_type_set.insert(types); },
            Some(existing) => return Err((types, existing)),
        }
    }
    Ok(())
}

pub fn get_result_type(field_type: &TokenStream2, is_good: bool) -> TokenStream2 {
    if is_good { quote! {Result<#field_type, Self>} } else { quote! {Result<Self, #field_type>} }
}