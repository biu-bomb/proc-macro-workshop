pub(crate) fn solution(fn_item: &mut syn::ItemFn) -> syn::Result<proc_macro2::TokenStream> {
    let mut visitor = crate::common::MatchVisitor {
        err: std::option::Option::None,
    };
    syn::visit_mut::visit_item_fn_mut(&mut visitor, fn_item);
    match visitor.err {
        Some(e) => syn::Result::Err(e),
        None => syn::Result::Ok(crate::common::to_token_stream(fn_item)),
    }
}
