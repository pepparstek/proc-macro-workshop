use proc_macro::{Span, TokenStream};
use quote::quote;
use syn::{parse_macro_input, spanned::Spanned, DeriveInput};

#[proc_macro_attribute]
pub fn bitfield(args: TokenStream, input: TokenStream) -> TokenStream {
    if !args.is_empty() {
        let args = parse_macro_input!(args as DeriveInput);
        eprintln!("{:#?}", args);
    }
    let ast = parse_macro_input!(input as DeriveInput);
    eprintln!("{:#?}", ast);
    let fields = if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(syn::FieldsNamed { ref named, .. }),
        ..
    }) = ast.data
    {
        named
    } else {
        unimplemented!();
    };

    let bs = fields.iter().map(|f| {
        if let syn::Field {
            ty:
                syn::Type::Path(syn::TypePath {
                    path: syn::Path { ref segments, .. },
                    ..
                }),
            ..
        } = f
        {
            segments
        } else {
            unimplemented!()
        };
    });

    eprintln!("{:?}", bs.collect::<Vec<_>>());
    let expanded = quote! {
        // pub trait Specifier {
        //     const BITS: u8;
        // }
        //
        // impl Specifier for #bs {}

    };
    expanded.into()
}
