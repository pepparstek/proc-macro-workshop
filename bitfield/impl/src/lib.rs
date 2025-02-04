use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

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

    let punctuated = fields.iter().map(|f| {
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
        }
    });

    // let idents = punctuated
    //     .into_iter()
    //     .map(|pathsegment| pathsegment.first().unwrap().ident.clone())
    //     .collect::<Vec<_>>()
    //     .clone();

    let traits = quote!(
        pub trait Specifier {
            const BITS: u8;
        }
    );

    let mut bs: Vec<syn::Ident> = Vec::with_capacity(64);
    let mut bits: Vec<u8> = Vec::with_capacity(64);
    for i in 1..=64 {
        let name = format!("B{}", i);
        let ident = syn::Ident::new(name.as_str(), proc_macro2::Span::call_site());
        bs.push(ident);
        bits.push(i);
    }

    let structs = quote! {
        #(pub struct #bs;)*
    };

    let impls = quote!(
        #(impl Specifier for #bs{
            const BITS: u8 = #bits;
        })*
    );

    let prog = quote!(
        #traits
        #structs
        #impls
    );
    prog.into()
}
