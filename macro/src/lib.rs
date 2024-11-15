mod style;

use style::ParsedVariants;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};

#[allow(clippy::missing_panics_doc)]
#[proc_macro_derive(StyleParser, attributes(property, parser, style_class))]
pub fn derive_style_parser(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let generics = input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let Data::Enum(e) = input.data else {
        panic!("StyleParser can be only derived to enum")
    };
    let ParsedVariants {
        idents,
        properties,
        parsers,
        style_classes,
    } = ParsedVariants::from(&e.variants);

    quote! {
        impl #impl_generics #name #ty_generics #where_clause {
            pub fn from_cow((key, value): (&std::borrow::Cow<'_, str>, &std::borrow::Cow<'_, str>)) -> Option<Self> {
                match key.as_ref() {
                    #( #properties => Some(Self::#idents(#parsers(value)?)), )*
                    unknown => None,
                }
            }

            fn apply_transition(s: floem::style::Style, key: &str, t: floem::style::Transition) -> floem::style::Style {
                match key {
                    #( #properties => s.transition(#style_classes, t), )*
                    invalid => {
                        log::error!("Invalid transition key '{invalid}'");
                        s
                    },
                }
            }
        }
    }
    .into()
}
