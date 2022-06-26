pub(super) fn solution(
    fields: &crate::common::FieldsType,
    origin_ident: &syn::Ident,
) -> syn::Result<proc_macro2::TokenStream> {
    let field_stream_vec: Vec<_> = fields
        .iter()
        .map(|f| {
            let ident = &f.ident.as_ref();
            let ident_string = ident.unwrap().to_string();
            quote::quote! {
                .field(#ident_string, &self.#ident)
            }
        })
        .collect();

    let origin_ident_string = origin_ident.to_string();
    syn::Result::Ok(quote::quote! {
        impl std::fmt::Debug for #origin_ident {
            fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
                fmt.debug_struct(#origin_ident_string)
                #(
                    #field_stream_vec
                )*
                .finish()
            }
        }
    })
}
