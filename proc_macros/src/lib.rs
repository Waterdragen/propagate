mod helper_fn;
mod bool_packing;

use proc_macro::TokenStream;
use proc_macro2::{Literal, TokenStream as TokenStream2};
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Variant};

use helper_fn::*;

#[proc_macro_derive(Good, attributes(good))]
pub fn derive_good(input: TokenStream) -> TokenStream {
    let DeriveInput { data, ident, .. } = parse_macro_input!(input);

    let variants: Vec<Variant>;
    let good_variants: Vec<Variant> = match data {
        Data::Enum(data) => {
            variants = data.variants.into_iter().collect();
            variants.iter().cloned()
                .filter(has_good_attribute)
                .filter(ensure_unit_or_tuple_struct)
                .collect()
        },
        _ => panic!("`Good` trait can only be derived for enums"),
    };

    if good_variants.is_empty() {
        panic!("`Good` trait must contain at least one `#[good]` attribute. Did you forget to mark a good variant?");
    }

    let grouped_variants = group_variant_by_type(&good_variants);

    let good_impls = grouped_variants.into_iter().map(|(fields, variants)| {
        let field_type = get_tuple_field_type(&fields, quote! {&'a});
        let field_type_mut = get_tuple_field_type(&fields, quote! {&'a mut});
        let field_type_once = get_tuple_field_type(&fields, quote! {});
        let (input, output) = get_any_field_input_and_output(&fields);
        let output_ref = match &fields {
            Fields::Unit => quote! { Box::leak(Box::new(())) },
            _ => output.clone(),
        };
        let match_rules_ref = variants.iter().map(|v| {
            let variant_name = &v.ident;
            quote! { #ident::#variant_name #input => Ok(#output_ref), }
        });
        let match_rules = variants.iter().map(|v| {
            let variant_name = &v.ident;
            quote! { #ident::#variant_name #input => Ok(#output), }
        });
        let body_ref = quote! {
            match self {
                #(#match_rules_ref)*
                _ => Err(self),
            }
        };
        let body = quote! {
            match self {
                #(#match_rules)*
                _ => Err(self),
            }
        };
        quote! {
            impl<'a> Good<#field_type> for &'a #ident {
                fn good(self) -> Result<#field_type, Self> {
                    #body_ref
                }
            }
            impl<'a> Good<#field_type_mut> for &'a mut #ident {
                fn good(self) -> Result<#field_type_mut, Self> {
                    #body_ref
                }
            }
            impl Good<#field_type_once> for #ident {
                fn good(self) -> Result<#field_type_once, Self> {
                    #body
                }
            }
        }
    });

    let good_indexes = variants.iter().map(has_good_attribute);
    let good_indexes = bool_packing::pack_bool(good_indexes);
    let good_indexes: Vec<Literal> = good_indexes
        .into_iter()
        .map(|num| { Literal::u8_unsuffixed(num) })
        .collect();
    let index_map: Vec<TokenStream2> = get_index_map(&ident, &variants);
    let good_index_impl = quote! {
        impl ::propagate::__private::__GoodIndex for #ident {
            fn __good_indexes(&self) -> &'static [u8] {
                &[#(#good_indexes),*]
            }
            fn __get_index(&self) -> usize {
                match self {
                    #(#index_map)*
                }
            }
        }
    };

    let output = quote! {
        #(#good_impls)*
        #good_index_impl
    };

    output.into()
}

#[proc_macro_derive(Bad, attributes(bad))]
pub fn derive_bad(input: TokenStream) -> TokenStream {
    let DeriveInput { data, ident, .. } = parse_macro_input!(input);

    let variants: Vec<Variant>;
    let bad_variants: Vec<Variant> = match data {
        Data::Enum(data) => {
            variants = data.variants.into_iter().collect();
            variants.iter().cloned()
                .filter(has_bad_attribute)
                .filter(ensure_unit_or_tuple_struct)
                .collect() }
        _ => panic!("`Bad` trait can only be derived for enums"),
    };

    if bad_variants.is_empty() {
        panic!("`Bad` trait must contain at least one `#[bad]` attribute. Did you forget to mark a bad variant?");
    }

    let grouped_variants = group_variant_by_type(&bad_variants);

    let bad_impls = grouped_variants.into_iter().map(|(fields, variants)| {
        let field_type = get_tuple_field_type(&fields, quote! {&'a});
        let field_type_mut = get_tuple_field_type(&fields, quote! {&'a mut});
        let field_type_once = get_tuple_field_type(&fields, quote! {});
        let (input, output) = get_any_field_input_and_output(&fields);
        let output_ref = match &fields {
            Fields::Unit => quote! { Box::leak(Box::new(())) },
            _ => output.clone(),
        };
        let match_rules_ref = variants.iter().map(|v| {
            let variant_name = &v.ident;
            quote! { #ident::#variant_name #input => Err(#output_ref), }
        });
        let match_rules = variants.iter().map(|v| {
            let variant_name = &v.ident;
            quote! { #ident::#variant_name #input => Err(#output), }
        });
        let body_ref = quote! {
            match self {
                #(#match_rules_ref)*
                _ => Ok(self),
            }
        };
        let body = quote! {
            match self {
                #(#match_rules)*
                _ => Ok(self),
            }
        };

        quote! {
            impl<'a> Bad<#field_type> for &'a #ident {
                fn bad(self) -> Result<Self, #field_type> {
                    #body_ref
                }
            }
            impl<'a> Bad<#field_type_mut> for &'a mut #ident {
                fn bad(self) -> Result<Self, #field_type_mut> {
                    #body_ref
                }
            }
            impl Bad<#field_type_once> for #ident {
                fn bad(self) -> Result<Self, #field_type_once> {
                    #body
                }
            }
        }
    });

    let bad_indexes = variants.iter().map(has_bad_attribute);
    let bad_indexes = bool_packing::pack_bool(bad_indexes);
    let bad_indexes: Vec<Literal> = bad_indexes
        .into_iter()
        .map(|num| { Literal::u8_unsuffixed(num) })
        .collect();
    let index_map: Vec<TokenStream2> = get_index_map(&ident, &variants);
    let bad_index_impl = quote! {
        impl <'a> ::propagate::__private::__BadIndex for #ident {
            fn __bad_indexes(&self) -> &'static [u8] {
                &[#(#bad_indexes),*]
            }
            fn __get_index(&self) -> usize {
                match self {
                    #(#index_map)*
                }
            }
        }
    };

    let output = quote! {
        #(#bad_impls)*
        #bad_index_impl
    };

    output.into()
}

