mod common;
mod solution2;
mod solution3;

#[proc_macro_derive(CustomDebug, attributes(debug))]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse_macro_input!(input as syn::DeriveInput);
    match solution1(&ast) {
        syn::Result::Ok(token_stream) => {
            return proc_macro::TokenStream::from(token_stream);
        },
        syn::Result::Err(e) => {
            return e.into_compile_error().into();
        }
    }
}

fn solution1(ast: &syn::DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let origin_ident = &ast.ident;
    let fields = crate::common::parse_fields(&ast)?;
    let mut token_stream = proc_macro2::TokenStream::new();
    // soluton2
    let solution2_stream = solution2::solution(fields, origin_ident)?;
    token_stream.extend(solution2_stream);

    token_stream = solution3::solution(fields, origin_ident)?;

    syn::Result::Ok(token_stream)
}
