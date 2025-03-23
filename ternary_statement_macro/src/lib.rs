use proc_macro::Ident;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::DataEnum;
use syn::{self, parse::Parse, parse_macro_input, Meta};

struct Si {
    pred: syn::Expr,
    then: syn::Expr,
    otherwise: syn::Expr,
}

impl Parse for Si {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let pred = input.parse()?;
        _ = input.parse::<syn::Token![?]>()?;
        let then = input.parse()?;
        _ = input.parse::<syn::Token![:]>()?;
        let otherwise = input.parse()?;

        Ok(Si {
            pred,
            then,
            otherwise,
        })
    }
}

impl ToTokens for Si {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let Si {
            pred,
            then,
            otherwise,
        } = self;

        tokens.extend(quote! {
            if Truthy::to_bool(#pred) { #then } else { #otherwise }
        })
    }
}

#[proc_macro]
pub fn si(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let tal = parse_macro_input!(input as Si);
    quote! { #tal }.into()
}

// #[derive(Truthyable) aera
fn extract_ident_from_attr(attr: &syn::Attribute) -> Option<String> {
    if let Meta::Path(path) = &attr.meta {
        path.get_ident().map(|i| i.to_string())
    } else {
        None
    }
}
#[proc_macro_derive(Truthyable, attributes(truthy, falsey))]
pub fn truthyable(input: proc_macro::TokenStream) -> TokenStream {
    let input: syn::DeriveInput = parse_macro_input!(input as syn::DeriveInput);
    let enumeracio: DataEnum = match &input.data {
        syn::Data::Enum(e) => e.clone(),
        _ => panic!("Expected an enum"),
    };

    let tru = "truthy";
    let fal = "falsey";
    let mut truthy_variants = vec![];
    let mut falsey_variants = vec![];

    for variant in enumeracio.variants {
        match variant
            .attrs
            .iter()
            .map(extract_ident_from_attr)
            .next()
            .flatten()
        {
            Some(s) if s == tru => truthy_variants.push(variant),
            Some(s) if s == fal => falsey_variants.push(variant),
            _ => panic!(
                "Variant '{}' of Truthyable enum '{}' does not have #[{tru}] or #[{fal}]",
                variant.ident, input.ident
            ),
        }
    }

    for v in &mut truthy_variants {
        v.attrs = vec![]
    }
    for v in &mut falsey_variants {
        v.attrs = vec![]
    }

    let enum_name = input.ident.clone();
    quote! {
        impl Truthy for #enum_name {
            fn to_bool(self) -> bool {
                match self {
                    #(Self::#truthy_variants => true,)*
                    #(Self::#falsey_variants => false,)*
                }
            }
        }
    }
    .into()
}
