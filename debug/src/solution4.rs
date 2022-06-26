pub(super) fn solution(
    fields: &crate::common::FieldsType,
    origin_ident: &syn::Ident,
    ast: &syn::DeriveInput,
) -> syn::Result<proc_macro2::TokenStream> {
    let mut generic = crate::common::parse_generic_type(ast);
    for g in generic.params.iter_mut() {
        if let syn::GenericParam::Type(t) = g {
            t.bounds.push(syn::parse_quote!(std::fmt::Debug));
        }
    }
    let (impl_generics, type_generics, where_clause) = generic.split_for_impl();
    let origin_ident_string = origin_ident.to_string();
    let debug_field_vec = debug_body_vec(fields)?;
    syn::Result::Ok(quote::quote! {
        impl #impl_generics std::fmt::Debug for #origin_ident #type_generics #where_clause {
            fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
                fmt.debug_struct(#origin_ident_string)
                #(
                    #debug_field_vec
                )*
                .finish()
            }
        }
    })
}

fn debug_body_vec(
    fields: &crate::common::FieldsType,
) -> syn::Result<Vec<proc_macro2::TokenStream>> {
    fields
        .iter()
        .map(|f| {
            let ident = &f.ident;
            let ident_string = ident.as_ref().unwrap().to_string();
            let mut format = "{:?}".to_string();
            if let std::option::Option::Some(customer_format) = crate::common::parse_format(f)? {
                format = customer_format;
            }
            syn::Result::Ok(quote::quote! {
                .field(#ident_string, &format_args!(#format, &self.#ident))
            })
        })
        .collect()
}
