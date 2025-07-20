use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, FieldsNamed};

fn todo_app_impl(input: DeriveInput) -> Result<proc_macro2::TokenStream, syn::Error> {    
   
}

#[proc_macro_attribute]
pub fn todo_app(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match todo_app_impl(input) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}
