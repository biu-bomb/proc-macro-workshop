pub(super) fn solution(fields: &crate::common::FielsType, origin_ident: &syn::Ident, ast: &syn::DeriveInput) -> syn::Result<proc_macro2::TokenStream> {

    let mut generics = crate::common::parse_generic_type(ast);
    let mut fields_type_names = vec![];
    let mut phantom_generic_type_names = vec![];
    for field in fields {
        if let std::option::Option::Some(field_name) = crate::common::parse_field_type_name(field)? {
            fields_type_names.push(field_name);
        }
        if let std::option::Option::Some(field_name) = crate::common::parse_phantom_generic_type_name(field)? {
            phantom_generic_type_names.push(field_name);
        }
    }
    for generic in generics.params.iter_mut() {
        if let syn::GenericParam::Type(t) = generic {
            let type_param_name = t.ident.to_string();
            if phantom_generic_type_names.contains(&type_param_name) 
            && !fields_type_names.contains(&type_param_name) {
                continue;
            }
            t.bounds.push(syn::parse_quote!(std::fmt::Debug));
        }
    } 
    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();
    let origin_ident_string = origin_ident.to_string();
    let fields_stream_vec = generate_field_stream_vec(fields)?;
    syn::Result::Ok(
        quote::quote! {
            impl #impl_generics std::fmt::Debug for #origin_ident #type_generics #where_clause {
                fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
                    fmt.debug_struct(#origin_ident_string)
                    #(
                        #fields_stream_vec
                    )*
                    .finish()
                }
            }
        }
    )
}


fn generate_field_stream_vec(fields: &crate::common::FielsType) -> syn::Result<Vec<proc_macro2::TokenStream>> {
    fields.iter().map(|f| {
        let ident = &f.ident;
        let ident_string = ident.as_ref().unwrap().to_string();
        let mut format = "{:?}".to_string();
        if let std::option::Option::Some(customer_format) = crate::common::parse_format(f)? {
            format = customer_format;
        }
        syn::Result::Ok(
            quote::quote! {
                .field(#ident_string, &format_args!(#format, &self.#ident))
            }
        )
    }).collect()
}