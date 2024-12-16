use proc_macro2::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use syn::{parse_macro_input, Attribute, Data, DeriveInput, ItemFn, LitChar};

struct TileChar(LitChar);

impl Parse for TileChar {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self(LitChar::parse(input)?))
    }
}

fn tile_tag(attrs: &[Attribute]) -> syn::Result<Option<LitChar>> {
    for attr in attrs {
        if attr.path().is_ident("tile") {
            return Ok(Some(attr.parse_args::<TileChar>()?.0));
        }
    }
    Ok(None)
}

#[proc_macro_derive(ParseTile, attributes(tile))]
pub fn derive_parsetile(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let ident = input.ident.clone();

    let enum_data = match input.data {
        Data::Enum(d) => d,
        _ => {
            return syn::Error::new(
                input.span(),
                "ParseTile derive macro can only be used on enum types",
            )
            .into_compile_error()
            .into()
        }
    };

    let mut parse_cases = TokenStream::new();

    for variant in enum_data.variants {
        let variant_ident = variant.ident;
        match tile_tag(&variant.attrs) {
            Ok(Some(ch)) => parse_cases.extend(std::iter::once(TokenStream::from(quote! {
                #ch => Some(Self::#variant_ident),
            }))),
            Ok(None) => {}
            Err(e) => return e.into_compile_error().into(),
        }
    }

    let expanded = quote! {
        impl ::aoc::map::ParseTile for #ident {
            fn from_char(ch: char) -> Option<Self> {
                match ch {
                    #parse_cases
                    _ => None
                }
            }
        }
    };

    TokenStream::from(expanded).into()
}

#[proc_macro_derive(DisplayTile, attributes(tile))]
pub fn derive_displaytile(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let ident = input.ident.clone();

    let enum_data = match input.data {
        Data::Enum(d) => d,
        _ => {
            return syn::Error::new(
                input.span(),
                "DisplayTile derive macro can only be used on enum types",
            )
            .into_compile_error()
            .into()
        }
    };

    let mut parse_cases = TokenStream::new();

    for variant in enum_data.variants {
        let variant_ident = variant.ident;
        match tile_tag(&variant.attrs) {
            Ok(Some(ch)) => parse_cases.extend(std::iter::once(TokenStream::from(quote! {
                Self::#variant_ident => #ch,
            }))),
            Ok(None) => {}
            Err(e) => return e.into_compile_error().into(),
        }
    }

    let expanded = quote! {
        impl ::aoc::map::DisplayTile for #ident {
            fn to_char(self) -> char {
                match self {
                    #parse_cases
                }
            }
        }
    };

    TokenStream::from(expanded).into()
}

#[proc_macro_attribute]
pub fn main(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let fn_name = &input.sig.ident.clone();

    quote! {
        #input

        fn main() -> impl ::std::process::Termination {
            use ::std::io::Read;
            static mut inp: ::std::string::String = ::std::string::String::new();
            let s = unsafe {
                ::std::io::stdin().read_to_string(&mut inp).unwrap();
                inp.as_ref()
            };
            #fn_name(s)
        }
    }
    .into()
}
