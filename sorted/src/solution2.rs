pub(crate) fn solution(item: &syn::Item) -> syn::Result<proc_macro2::TokenStream> {
    match item {
        syn::Item::Enum(node) => crate::solution3::solution(node),
        _ => syn::Result::Err(syn::Error::new(
            proc_macro2::Span::call_site(),
            "expected enum or match expression",
        )),
    }
}
