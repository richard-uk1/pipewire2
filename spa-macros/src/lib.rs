use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemStruct, parse_macro_input};

/// Implement `EncodePod` by pushing a struct with each field in sequence.
#[proc_macro_derive(PodStruct)]
pub fn derive_pod(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemStruct);
    let ident = &input.ident;

    let encode_fields = match &input.fields {
        syn::Fields::Named(fields_named) => {
            let fields = fields_named.named.iter().map(|field| {
                let field_ident = field.ident.as_ref().unwrap();
                quote! ( self.#field_ident.encode(dst) )
            });
            quote! {
                {
                    #(#fields;)*
                }
            }
        }
        syn::Fields::Unnamed(fields_unnamed) => {
            let fields = fields_unnamed
                .unnamed
                .iter()
                .enumerate()
                .map(|(idx, _)| quote! ( self.#idx.encode(dst)));
            quote! {
                (
                    #(#fields;)*
                )
            }
        }
        syn::Fields::Unit => quote!(),
    };

    let decode_fields = match &input.fields {
        syn::Fields::Named(fields_named) => {
            let fields = fields_named.named.iter().map(|field| {
                let field_ident = field.ident.as_ref().unwrap();
                let ty = &field.ty;
                quote! ( #field_ident: #ty::decode(src) )
            });
            quote! {
                {
                    #(#fields?,)*
                }
            }
        }
        syn::Fields::Unnamed(fields_unnamed) => {
            let fields = fields_unnamed.unnamed.iter().map(|field| {
                let ty = &field.ty;
                quote! ( #ty::decode(src))
            });
            quote! {
                (
                    #(#fields?,)*
                )
            }
        }
        syn::Fields::Unit => quote!(),
    };

    quote! {
        impl ::spa::EncodePod for #ident {
            fn encode(&self, dst: &mut ::spa::PodBuf) {
                dst.put_struct(|dst| {
                    #encode_fields
                });
            }
        }

        impl ::spa::DecodePod for #ident {
            fn decode(src: &mut ::bytes::Bytes) -> Result<Self, ::spa::DecodePodError> {
                use ::spa::PodDecoder;
                src.decode_struct(|src| {
                    Ok(Self
                        #decode_fields
                    )
                })
            }
        }

    }
    .into()
}
