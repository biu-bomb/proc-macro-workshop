mod common;
mod solution2;
mod solution3;
mod solution4;
mod solution56;
mod solution7;
mod solution8;

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
    // soluton2
    _ = solution2::solution(fields, origin_ident)?;

    _ = solution3::solution(fields, origin_ident)?;

    _ = solution4::solution(fields, origin_ident, ast)?;

    _ = solution56::solution(fields, origin_ident, ast)?;

    _ = solution7::soution(fields, origin_ident, ast)?;

    let token_stream = solution8::solution(fields, origin_ident, ast)?;

    syn::Result::Ok(token_stream)
}
