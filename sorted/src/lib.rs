mod common;
mod solution1;
mod solution2;
mod solution3;
mod solution5;

#[proc_macro_attribute]
pub fn sorted(
    _args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let item = syn::parse_macro_input!(input as syn::Item);
    match solution1::solution(&item) {
        syn::Result::Ok(stream) => stream,
        syn::Result::Err(e) => {
            let mut res = e.into_compile_error();
            res.extend(crate::common::to_token_stream(item));
            res
        }
    }
    .into()
}

#[proc_macro_attribute]
pub fn check(
    _args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let mut fn_item = syn::parse_macro_input!(input as syn::ItemFn);
    match solution5::solution(&mut fn_item) {
        syn::Result::Ok(stream) => stream,
        syn::Result::Err(e) => {
            let mut res = e.into_compile_error();
            res.extend(crate::common::to_token_stream(fn_item));
            res
        }
    }
    .into()
}
