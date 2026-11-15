use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(Pod)]
pub fn derive_pod(_input: TokenStream) -> TokenStream {
    quote!().into()
}
