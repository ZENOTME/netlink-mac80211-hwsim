use itertools::Itertools;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, Fields};

/// Used to generate emitable trait implementation for a message.
///
/// # Note
/// The emit way is to emit the message according the field order.
///
/// # Limit
///
/// 1. The struct must be a named struct.
/// 2. All field must be a type that implements `Emitable`.
#[proc_macro_attribute]
pub fn generate_emitable(_: TokenStream, input: TokenStream) -> TokenStream {
    let struct_item: syn::ItemStruct = parse_macro_input!(input as syn::ItemStruct);
    let struct_name_ident = struct_item.ident.clone();

    let field_idents = match &struct_item.fields {
        Fields::Named(fields) => fields
            .named
            .iter()
            .map(|field| field.ident.as_ref().unwrap().clone())
            .collect_vec(),
        Fields::Unit => vec![],
        _ => panic!("This macro only supports named struct"),
    };

    let emitable = generate_emitable_impl(struct_name_ident.clone(), field_idents);

    let mut res = struct_item.into_token_stream();
    res.extend(emitable);

    res.into()
}

fn generate_emitable_impl(
    struct_name_ident: syn::Ident,
    field_idents: Vec<syn::Ident>,
) -> TokenStream2 {
    // Generate emit statements
    let emit_statements = field_idents.iter().map(|field_ident| {
        quote! {
            let attr_len = self.#field_ident.buffer_len();
            assert_eq!(attr_len % NLA_ALIGNTO ,0);
            let end = start + attr_len;
            self.#field_ident.emit(&mut buffer[start..end]);
            start = end;
        }
    });

    // Generate emit function
    let emit_function = quote! {
        fn emit(&self, buffer: &mut [u8]) {
            let mut start = 0;
            #(#emit_statements)*
        }
    };

    // Generate buffer_len statements
    let buffer_len_statements = field_idents.iter().map(|field_ident| {
        quote! {
            let attr_len = self.#field_ident.buffer_len();
            assert_eq!(attr_len % NLA_ALIGNTO ,0);
            len += attr_len;
        }
    });

    // Generate buffer_len function
    let buffer_len_function = quote! {
        fn buffer_len(&self) -> usize {
            let mut len = 0;
            #(#buffer_len_statements)*
            len
        }
    };

    // Generate implementation of emitable trait
    let impl_emitable = quote! {
        impl Emitable for #struct_name_ident {
            #emit_function
            #buffer_len_function
        }
    };

    impl_emitable.into()
}

/// Used to generate a parse function for a message.
///
/// # Note
///
/// The parse way is to parse the message according the field order.
/// If you want to parse the message in another way, you can implement the parse function by yourself.
///
/// # Limit
///
/// 1. The struct must be a named struct.
/// 2. All field must be a type that implements `Parseable<NlaBuffer<&'a T>>`.
/// 3. The struct with only one bool field is not supported.
#[proc_macro_attribute]
pub fn generate_parse(_: TokenStream, input: TokenStream) -> TokenStream {
    let struct_item: syn::ItemStruct = parse_macro_input!(input as syn::ItemStruct);
    let struct_name_ident = struct_item.ident.clone();

    let field_idents = match &struct_item.fields {
        Fields::Named(fields) => fields
            .named
            .iter()
            .map(|field| field.ident.as_ref().unwrap().clone())
            .collect_vec(),
        Fields::Unit => vec![],
        _ => panic!("This macro only supports named struct"),
    };

    let field_types = match &struct_item.fields {
        Fields::Named(fields) => fields
            .named
            .iter()
            .map(|field| field.ty.clone())
            .collect_vec(),
        Fields::Unit => vec![],
        _ => panic!("This macro only supports named struct"),
    };

    let parse = geneate_parse_impl(struct_name_ident.clone(), field_idents, field_types);

    let mut res = struct_item.into_token_stream();
    res.extend(parse);

    res.into()
}

fn geneate_parse_impl(
    struct_name_ident: syn::Ident,
    field_idents: Vec<syn::Ident>,
    field_types: Vec<syn::Type>,
) -> TokenStream2 {
    // Generate parse statements
    let parse_statements =
        field_idents
            .iter()
            .zip_eq(field_types.iter())
            .map(|(field_ident, field_type)| {
                quote! {
                    let #field_ident = <#field_type>::parse(&mut iter)?;
                }
            });

    // Generate parse function
    let parse_function = quote! {
        fn parse(buf: &[u8]) -> Result<Self, DecodeError>{
            let mut iter = NlasIterator::new(buf);
            #(#parse_statements)*
            Ok(#struct_name_ident{
                #(#field_idents),*
            })
        }
    };

    // Impl the parse function
    quote! {
        impl  #struct_name_ident {
            #parse_function
        }
    }
    .into()
}
