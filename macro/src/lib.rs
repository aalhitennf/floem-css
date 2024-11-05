mod style;

use style::{parse_enum_variant, ParsedVariants};

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};

#[allow(clippy::missing_panics_doc)]
#[proc_macro_derive(StyleParser, attributes(key, parser, prop))]
pub fn derive_style_parser(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident.clone();
    let generics = input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let Data::Enum(e) = input.data else {
        panic!("StyleParser can be only derived to enum")
    };
    let ParsedVariants {
        idents,
        names,
        parsers,
        props,
    } = e.variants.iter().filter_map(parse_enum_variant).fold(
        ParsedVariants::default(),
        |mut v, p| {
            v.add(p);
            v
        },
    );
    quote! {
        impl #impl_generics TryFrom<&StyleProperty> for #name #ty_generics #where_clause {
            type Error = crate::style::parser::StyleError;
            fn try_from(value: &crate::style::parser::StyleProperty) -> Result<Self, Self::Error> {
                match value.key.as_str() {
                    #( #names => Ok(#name::#idents(#parsers(&value.value)?)), )*
                    val @ _ => Err(crate::style::parser::StyleError::new("Unknown style key", val)),
                }
            }
        }

        impl #impl_generics #name #ty_generics #where_clause {
            pub fn apply_transition(s: floem::style::Style, key: &str, t: floem::style::Transition) -> floem::style::Style {
                match key {
                    #( #names => s.transition(#props, t), )*
                    val @ _ => {
                        log::error!("Invalid transition key {val}");
                        s
                    },
                }
            }
        }
    }
    .into()
}
