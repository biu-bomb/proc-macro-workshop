pub(crate) fn solution(item: &syn::ItemEnum) -> syn::Result<proc_macro2::TokenStream> {
    let mut names: Vec<(String, &dyn quote::ToTokens)> = Vec::new();
    for i in item.variants.iter() {
        names.push((i.ident.to_string(), &i.ident));
    }
    match crate::common::check_order(names) {
        Some(e) => syn::Result::Err(e),
        None => syn::Result::Ok(crate::common::to_token_stream(item)),
    }
}
