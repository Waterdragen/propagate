#![no_std]

mod helper_fn;
mod bool_packing;

extern crate alloc;

use proc_macro::TokenStream;
use proc_macro2::{Literal, TokenStream as TokenStream2};
use alloc::vec::Vec;
use hashbrown::HashSet;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Field, Fields, Generics, Type, Variant};

use helper_fn::*;

#[proc_macro_derive(Propagate, attributes(good, bad))]
pub fn derive_propagate(input: TokenStream) -> TokenStream {
    let trait_path = quote! {::propagate::};

    let DeriveInput { data, ident, generics, .. } = parse_macro_input!(input);

    let variants: Vec<Variant> = if let Data::Enum(data) = data {
        data.variants.into_iter().collect()
    } else {
        panic!("`Propagate` procedural macro (for `Good` or `Bad` traits) can only be derived for enums");
    };
    let Generics {
        params,
        where_clause,
        lt_token,
        gt_token,
    } = generics;
    let trailing_comma = if params.is_empty() { quote! {} } else { quote! {,} };

    let good_variants: Vec<&Variant> = variants
        .iter()
        .filter(|v| has_good_attribute(v))
        .filter(|v| ensure_unit_or_tuple_struct(v))
        .collect();
    let bad_variants: Vec<&Variant> = variants
        .iter()
        .filter(|v| has_bad_attribute(v))
        .filter(|v| ensure_unit_or_tuple_struct(v))
        .collect();

    if good_variants.is_empty() && bad_variants.is_empty() {
        panic!("Enum must contain at least one `#[good]` or `#[bad]` attribute. \
                Did you forget to mark a good or bad variant?");
    }

    let grouped_good_variants = group_variant_ref_by_type(&good_variants);
    let grouped_bad_variants = group_variant_ref_by_type(&bad_variants);

    let grouped_good_variants_iter = grouped_good_variants.iter()
        .map(|(fields, variants)| (true, fields, variants));
    let grouped_bad_variants_iter = grouped_bad_variants.iter()
        .map(|(fields, variants)| (false, fields, variants));

    let grouped_variants_iter = grouped_good_variants_iter.chain(grouped_bad_variants_iter);

    match (validate_grouped_variants(grouped_good_variants.keys()), validate_grouped_variants(grouped_bad_variants.keys())) {
        (Ok(_), Ok(_)) => {},
        (_, Err((a, b))) | (Err((a, b)), _) => {
            panic!("Types `{a:?}` and `{b:?}` are ambiguous. Cannot infer types for both tuple and n-args variants.");
        }
    }

    let mut tuple_like_type_set = HashSet::new();
    for fields in grouped_good_variants.keys() {
        let fields = if let Fields::Unnamed(fields) = fields {fields} else {continue};
        let fields: Vec<&Field> = fields.unnamed.iter().collect();
        let types: Vec<&Type>;
        if fields.len() == 1 {
            let tuple_ty = if let Type::Tuple(tuple_ty) = &fields[0].ty {tuple_ty} else {continue};
            types = tuple_ty.elems.iter().collect::<Vec<_>>();
        } else {
            types = fields.iter().map(|field| &field.ty).collect::<Vec<_>>();
        }
        match tuple_like_type_set.get(&types) {
            None => { tuple_like_type_set.insert(types); },
            Some(types) => panic!("Types `{types:?}` and `{:?}` are ambiguous. Cannot infer types for both tuple and n-args variants.", types),
        }
    }

    let lifetime = quote! {'p};
    let (borrow, borrow_mut, owned) = (quote! {& #lifetime}, quote! {& #lifetime mut}, quote! {});
    let generic_ref = quote! {<#lifetime #trailing_comma #params>};
    let generic = quote! {#lt_token #params #gt_token};
    let impls = grouped_variants_iter.clone().map(|(is_good, fields, variants)| {
        let field_type_ref = get_tuple_field_type(fields, &borrow);
        let field_type_mut = get_tuple_field_type(fields, &borrow_mut);
        let field_type = get_tuple_field_type(fields, &owned);

        let result_type_ref = get_result_type(&field_type_ref, is_good);
        let result_type_mut = get_result_type(&field_type_mut, is_good);
        let result_type = get_result_type(&field_type, is_good);

        let (trait_name, method, keep_variant, dump_variant) =
            if is_good { (quote! {#trait_path Good}, quote! {good}, quote! {Ok}, quote! {Err})}
            else { (quote! {#trait_path Bad}, quote! {bad}, quote! {Err}, quote! {Ok})};

        let (input, output) = get_any_field_input_and_output(fields);
        let match_rules = variants.iter().map(|v| {
            let variant_name = &v.ident;
            quote! { #ident::#variant_name #input => #keep_variant(#output), }
        });
        let body = quote! {
            match self {
                #(#match_rules)*
                _ => #dump_variant(self),
            }
        };
        let impl_owned = quote! {
                impl #generic #trait_name <#field_type> for #ident #generic #where_clause {
                    fn #method (self) -> #result_type {
                        #body
                    }
                }
            };
        match fields {
            Fields::Unit => impl_owned,
            _ =>
            quote! {
                impl #generic_ref #trait_name <#field_type_ref> for & #lifetime #ident #generic #where_clause {
                    fn #method (self) -> #result_type_ref {
                        #body
                    }
                }
                impl #generic_ref #trait_name <#field_type_mut> for & #lifetime mut #ident #generic #where_clause {
                    fn #method (self) -> #result_type_mut {
                        #body
                    }
                }
                #impl_owned
            }
        }

    });

    let index_matcher: Vec<TokenStream2> = get_index_matcher(&ident, &variants);
    let get_index_impl = quote! {
        impl #generic #trait_path __private::__GetIndex for #ident #generic #where_clause {
            fn get_index(&self) -> usize {
                match self {
                    #(#index_matcher)*
                }
            }
        }
    };

    let good_attribute_iter = variants.iter().map(has_good_attribute);
    let good_packed = bool_packing::pack_bool(good_attribute_iter);
    let good_packed_lit: Vec<Literal> = good_packed
        .iter()
        .map(|num| { Literal::u8_unsuffixed(*num) })
        .collect();
    let good_index_impl = quote! {
        impl #generic #trait_path __private::__GoodIndex for #ident #generic #where_clause {
            fn good_indexes(&self) -> &'static [u8] {
                &[#(#good_packed_lit),*]
            }
        }
    };

    let bad_attribute_iter = variants.iter().map(has_bad_attribute);
    let bad_packed = bool_packing::pack_bool(bad_attribute_iter);
    let bad_packed_lit: Vec<Literal> = bad_packed
        .iter()
        .map(|num| { Literal::u8_unsuffixed(*num) })
        .collect();
    let bad_index_impl = quote! {
        impl #generic #trait_path __private::__BadIndex for #ident #generic #where_clause {
            fn bad_indexes(&self) -> &'static [u8] {
                &[#(#bad_packed_lit),*]
            }
        }
    };

    let from_good_bad_impls = grouped_variants_iter.clone()
        .filter(|(_, _, variants)| variants.len() == 1)
        .map(|(is_good, fields, variants)| {
        let field_type = get_tuple_field_type(fields, &owned);
        let variant_name = &variants[0].ident;
        let (trait_name, method) =
            if is_good {(quote! {FromGood}, quote! {from_good})}
            else {(quote! {FromBad}, quote! {from_bad})};
        let instantiate = match fields {
            Fields::Unit => quote! {#ident::#variant_name},
            Fields::Unnamed(unnamed) if unnamed.unnamed.len() == 1 => {
                quote! {#ident::#variant_name (_v)}
            },
            Fields::Unnamed(_) => {
                let (input, _) = get_any_field_input_and_output(fields);
                quote! {
                    let #input = _v;
                    #ident::#variant_name #input
                }
            }
            Fields::Named(_) => unreachable!(),
        };

        quote! {
            impl #generic #trait_path #trait_name <#field_type> for #ident #generic #where_clause {
                fn #method(_v: #field_type) -> Self {
                    #instantiate
                }
            }
        }
    });

    let two_states_impl: Option<_> =
        if grouped_good_variants.len() == 1 && grouped_bad_variants.len() == 1 &&
            variants.len() == 2 && &good_packed != &bad_packed {
        Some(
            quote! {unsafe impl #generic #trait_path ExactlyTwoDistinctVariants for #ident #generic #where_clause {}}
        )
    } else {
        None
    };

    let output = quote! {
        #(#impls)*
        #get_index_impl
        #good_index_impl
        #bad_index_impl
        #(#from_good_bad_impls)*
        #two_states_impl
    };
    output.into()
}

fn get_result_type(field_type: &TokenStream2, is_good: bool) -> TokenStream2 {
    if is_good { quote! {Result<#field_type, Self>} } else { quote! {Result<Self, #field_type>} }
}
