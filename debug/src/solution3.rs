pub(super) fn solution(
    fields: &crate::common::FieldsType,
    origin_ident: &syn::Ident,
) -> syn::Result<proc_macro2::TokenStream> {
    let fiels_stream_vec_result: syn::Result<Vec<proc_macro2::TokenStream>> = fields
        .iter()
        .map(|f| {
            let ident = &f.ident;
            let ident_string = ident.as_ref().unwrap().to_string();
            let mut format = "{:?}".to_string();
            if let Some(customer_format) = crate::common::parse_format(f)? {
                format = customer_format;
            }
            syn::Result::Ok(quote::quote! {
                .field(#ident_string, &format_args!(#format, self.#ident))
            })
        })
        .collect();
    let fiels_stream_vec = fiels_stream_vec_result?;
    let origin_ident_string = origin_ident.to_string();
    syn::Result::Ok(quote::quote! {
        impl std::fmt::Debug for #origin_ident {
            fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
                fmt.debug_struct(#origin_ident_string)
                #(
                    #fiels_stream_vec
                )*
                .finish()
            }
        }
    })
}
