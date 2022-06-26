pub(crate) fn solution(item: &syn::Item) -> syn::Result<proc_macro2::TokenStream> {
    crate::solution2::solution(item)
}
