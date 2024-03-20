use proc_macro::TokenStream;

use darling::FromDeriveInput;
use proc_macro2::Ident;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, FieldsNamed, FieldsUnnamed, Generics};

#[derive(FromDeriveInput)]
#[darling(attributes(abi))]
struct Parameters {
    name: String,
}

#[proc_macro_derive(Abi, attributes(abi))]
pub fn derive_abi(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let parameters = Parameters::from_derive_input(&ast)
        .expect("Type deriving `Abi` should have a `#[name = \"...\"]` attribute.");
    let ident = &ast.ident;
    let generics = &ast.generics;

    let Data::Struct(data) = &ast.data else {
        panic!("Item deriving `Abi` should be a struct.")
    };

    match &data.fields {
        Fields::Named(fields) => impl_abi_for_named(parameters, ident, generics, fields),
        Fields::Unnamed(fields) => impl_abi_for_unnamed(parameters, ident, generics, fields),
        Fields::Unit => impl_abi_for_unit(parameters, ident, generics),
    }
    .into()
}

fn impl_abi_for_named(
    parameters: Parameters,
    ident: &Ident,
    generics: &Generics,
    fields: &FieldsNamed,
) -> proc_macro2::TokenStream {
    let name = parameters.name;
    let field_names = &fields
        .named
        .iter()
        .map(|field| field.ident.as_ref().unwrap())
        .collect::<Vec<_>>();

    quote! {
        impl #generics abi::Abi for #ident #generics {
            fn descriptor() -> Cow<'static, str> {
                #name.into()
            }

            fn from_bytes(bytes: &mut impl Iterator<Item = u8>) -> abi::Result<Self> {
                Ok(Self {
                    #( #field_names: <_ as abi::Abi>::from_bytes(bytes)?, ) *
                })
            }

            fn to_bytes(self) -> impl Iterator<Item = u8> {
                iter::empty()
                    #( .chain(abi::Abi::to_bytes(self.#field_names)) ) *
            }
        }
    }
}

fn impl_abi_for_unnamed(
    parameters: Parameters,
    ident: &Ident,
    generics: &Generics,
    fields: &FieldsUnnamed,
) -> proc_macro2::TokenStream {
    let name = parameters.name;
    let field_indices = &(0..fields.unnamed.len())
        .map(syn::Index::from)
        .collect::<Vec<_>>();

    quote! {
        impl #generics abi::Abi for #ident #generics {
            fn descriptor() -> Cow<'static, str> {
                #name.into()
            }

            fn from_bytes(bytes: &mut impl Iterator<Item = u8>) -> abi::Result<Self> {
                Ok(Self {
                    #( #field_indices: <_ as abi::Abi>::from_bytes(bytes)?, ) *
                })
            }

            fn to_bytes(self) -> impl Iterator<Item = u8> {
                iter::empty()
                    #( .chain(abi::Abi::to_bytes(self.#field_indices)) ) *
            }
        }
    }
}

fn impl_abi_for_unit(
    parameters: Parameters,
    ident: &Ident,
    generics: &Generics,
) -> proc_macro2::TokenStream {
    let name = parameters.name;

    quote! {
        impl #generics abi::Abi for #ident #generics {
            fn descriptor() -> Cow<'static, str> {
                #name.into()
            }

            fn from_bytes(_bytes: &mut impl Iterator<Item = u8>) -> abi::Result<Self> {
                Ok(Self)
            }

            fn to_bytes(self) -> impl Iterator<Item = u8> {
                iter::empty()
            }
        }
    }
}
