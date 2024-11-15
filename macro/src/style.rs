use proc_macro2::Span;
use quote::ToTokens;
use smallvec::SmallVec;
use syn::{punctuated::Punctuated, token::Comma, Ident, LitStr, Variant};

const SIZE: usize = 64;

pub struct ParsedVariants<'a> {
    pub(crate) idents: SmallVec<[&'a Ident; SIZE]>,
    pub(crate) properties: SmallVec<[LitStr; SIZE]>,
    pub(crate) parsers: SmallVec<[proc_macro2::Ident; SIZE]>,
    pub(crate) style_classes: SmallVec<[Ident; SIZE]>,
}

impl<'a> ParsedVariants<'a> {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            idents: SmallVec::with_capacity(capacity),
            properties: SmallVec::with_capacity(capacity),
            parsers: SmallVec::with_capacity(capacity),
            style_classes: SmallVec::with_capacity(capacity),
        }
    }

    pub fn add(&mut self, var: ParsedVariant<'a>) {
        self.idents.push(var.ident);
        self.properties.push(var.property);
        self.parsers.push(var.parser);
        self.style_classes.push(var.style_class);
    }
}

pub struct ParsedVariant<'a> {
    ident: &'a Ident,
    property: LitStr,
    parser: proc_macro2::Ident,
    style_class: Ident,
}

impl<'a> From<&'a Punctuated<Variant, Comma>> for ParsedVariants<'a> {
    fn from(value: &'a Punctuated<Variant, Comma>) -> Self {
        let mut variants = ParsedVariants::with_capacity(value.len());
        for variant in value.iter().map(parse_enum_variant) {
            variants.add(variant);
        }
        variants
    }
}

pub fn parse_enum_variant(variant: &Variant) -> ParsedVariant<'_> {
    let property = find_property(variant);
    let parser = find_parser(variant);
    let style_class = find_style_class(variant);
    ParsedVariant {
        ident: &variant.ident,
        property,
        parser,
        style_class,
    }
}

fn find_style_class(variant: &Variant) -> Ident {
    let style_class_attr = variant
        .attrs
        .iter()
        .find(|a| a.path().is_ident("style_class"))
        .unwrap_or_else(|| panic!("Missing style_class attribute for {}", variant.ident));
    style_class_attr.parse_args::<Ident>().unwrap_or_else(|e| {
        panic!(
            "style_class attribute must be ident: {}\n{e}",
            style_class_attr.to_token_stream()
        )
    })
}

fn find_property(variant: &Variant) -> LitStr {
    let property_attr = variant
        .attrs
        .iter()
        .find(|a| a.path().is_ident("property"))
        .unwrap_or_else(|| {
            panic!("Missing property attribute for {}", variant.ident);
        });
    property_attr.parse_args::<LitStr>().unwrap_or_else(|e| {
        panic!(
            "property attribute must be string literal: {}\n{e}",
            property_attr.to_token_stream()
        )
    })
}

fn find_parser(variant: &Variant) -> Ident {
    variant
        .attrs
        .iter()
        .find(|a| a.path().is_ident("parser"))
        .and_then(|a| {
            a.parse_args::<LitStr>()
                .ok()
                .map(|lit| Ident::new(&lit.value(), Span::call_site()))
        })
        .unwrap_or_else(|| panic!("Missing parser fn for {}", variant.ident))
}
