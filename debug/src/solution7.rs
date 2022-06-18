pub(super) fn soution(fields: &crate::common::FielsType, origin_ident: &syn::Ident, ast: &syn::DeriveInput) -> syn::Result<proc_macro2::TokenStream> {

    let mut origin_field_type_names = vec![];
    let mut phantom_generic_type_names = vec![];
    for field in fields.iter() {
        if let Some(origin_field_type_name) = crate::common::parse_field_type_name(field)? {
            origin_field_type_names.push(origin_field_type_name);
        }
        if let Some(phantom_generic_type_name) = crate::common::parse_phantom_generic_type_name(field)? {
            phantom_generic_type_names.push(phantom_generic_type_name);
        }
    }
    let associated_generics_type_map = crate::common::parse_generic_associated_types(ast);
    let mut generics = crate::common::parse_generic_type(ast);
    for generic in generics.params.iter_mut() {
        if let syn::GenericParam::Type(t) = generic {
            let type_name = t.ident.to_string();
            if phantom_generic_type_names.contains(&type_name) && !origin_field_type_names.contains(&type_name) {
                continue;
            }
            if associated_generics_type_map.contains_key(&type_name) && !origin_field_type_names.contains(&type_name) {
                continue;
            }
            t.bounds.push(syn::parse_quote!(std::fmt::Debug));
        }
    }
    generics.make_where_clause();
    for (_, associated_types) in associated_generics_type_map {
        for associated_type in associated_types {
            generics
                .where_clause
                .as_mut()
                .unwrap()
                .predicates
                .push(syn::parse_quote!(#associated_type:std::fmt::Debug));
        }
    } 

    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();
    let fields_strea_vec = generate_field_stream_vec(fields)?;
    let origin_ident_string = origin_ident.to_string();
    syn::Result::Ok(
        quote::quote! {
            impl #impl_generics std::fmt::Debug for #origin_ident #type_generics #where_clause {
                fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
                    fmt.debug_struct(#origin_ident_string)
                    #(
                        #fields_strea_vec
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
        if let Some(customer_format) = crate::common::parse_format(f)? {
            format = customer_format;
        }
        syn::Result::Ok(
            quote::quote! {
                .field(#ident_string, &format_args!(#format, &self.#ident))
            }
        )
    }).collect()
}