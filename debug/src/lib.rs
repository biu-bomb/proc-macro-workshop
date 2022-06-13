#[proc_macro_derive(CustomDebug)]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    solution1(input)
}

fn solution1(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let _files = syn::parse_macro_input!(input as syn::DeriveInput);
    let token_stream = proc_macro2::TokenStream::new();

    proc_macro::TokenStream::from(token_stream)
}
