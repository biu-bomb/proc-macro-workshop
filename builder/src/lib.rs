mod common;
mod solution2;
mod solution3;
mod solution45;
mod solution6;

#[proc_macro_derive(Builder)]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    solution1(input)
}

fn solution1(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse_macro_input!(input as syn::DeriveInput);
    let fields = {
        match common::parse_fields(&ast) {
            Ok(f) => f,
            Err(_e) => std::panic!(std::stringify!(_e))
        }
    };
    
    let origin_ident = &ast.ident;
    let builder_ident = &quote::format_ident!("{}Builder", origin_ident);
    let mut token_stream = proc_macro2::TokenStream::new();

    // solution2 
    let solution2_stream = solution2::solution(fields, origin_ident, builder_ident);
    token_stream.extend(solution2_stream);

    // solution3
    let solution3_stream = solution3::soultion(fields, builder_ident);
    token_stream.extend(solution3_stream);

    // solution45
    let solution45_stream = solution45::solution(fields, builder_ident, origin_ident);
    token_stream.extend(solution45_stream);

    // solution6
    token_stream = solution6::solution(fields, builder_ident, origin_ident);

    proc_macro::TokenStream::from(token_stream)




}
