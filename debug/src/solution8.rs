pub(super) fn solution(
    fields: &crate::common::FieldsType,
    origin_ident: &syn::Ident,
    ast: &syn::DeriveInput,
) -> syn::Result<proc_macro2::TokenStream> {
    let mut generics = crate::common::parse_generic_type(ast);
    let customer_debug = crate::common::parse_customer_debug(ast)?;
    if customer_debug.is_none() {
        let associated_type_name_map = crate::common::parse_generic_associated_types(ast);
        let (field_generic_type_names, phantom_generic_type_names) = parse_type_names(fields)?;
        for generic in generics.params.iter_mut() {
            if let syn::GenericParam::Type(t) = generic {
                let generic_name = t.ident.to_string();
                if phantom_generic_type_names.contains(&generic_name)
                    && !field_generic_type_names.contains(&generic_name)
                {
                    continue;
                }
                if associated_type_name_map.contains_key(&generic_name)
                    && !field_generic_type_names.contains(&generic_name)
                {
                    continue;
                }
                t.bounds.push(syn::parse_quote!(std::fmt::Debug));
            }
        }
        generics.make_where_clause();
        for (_, associated_types) in associated_type_name_map {
            for associated_type in associated_types {
                generics
                    .where_clause
                    .as_mut()
                    .unwrap()
                    .predicates
                    .push(syn::parse_quote!(#associated_type: std::fmt::Debug));
            }
        }
    } else {
        generics.make_where_clause();
        generics
            .where_clause
            .as_mut()
            .unwrap()
            .predicates
            .push(syn::parse_str(customer_debug.unwrap().as_str()).unwrap());
    }
    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();
    let origin_ident_string = origin_ident.to_string();
    let fields_stream_vec = generate_field_stream_vec(fields)?;
    syn::Result::Ok(quote::quote! {
        impl #impl_generics std::fmt::Debug for #origin_ident #type_generics #where_clause {
            fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
                fmt.debug_struct(#origin_ident_string)
                #(
                    #fields_stream_vec
                )*
                .finish()
            }
        }
    })
}

fn generate_field_stream_vec(
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

fn parse_type_names(fields: &crate::common::FieldsType) -> syn::Result<(Vec<String>, Vec<String>)> {
    let mut field_generic_type_names = vec![];
    let mut phantom_generic_type_names = vec![];
    for field in fields.iter() {
        if let std::option::Option::Some(field_generic_type_name) =
            crate::common::parse_field_type_name(field)?
        {
            field_generic_type_names.push(field_generic_type_name);
        }
        if let std::option::Option::Some(phantom_generic_type_name) =
            crate::common::parse_phantom_generic_type_name(field)?
        {
            phantom_generic_type_names.push(phantom_generic_type_name);
        }
    }
    syn::Result::Ok((field_generic_type_names, phantom_generic_type_names))
}
