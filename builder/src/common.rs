use syn::spanned::Spanned;

pub(crate) type FieldsType = syn::punctuated::Punctuated<syn::Field, syn::Token!(,)>;

pub(super) fn parse_fields(ast: &syn::DeriveInput) -> syn::Result<&FieldsType> {
    if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(syn::FieldsNamed { ref named, .. }),
        ..
    }) = ast.data
    {
        return syn::Result::Ok(named);
    }
    let err = syn::Error::new_spanned(ast, "parse fields error");
    syn::Result::Err(err)
}

pub(crate) fn option_type(ty: &syn::Type) -> std::option::Option<&syn::Type> {
    option_type_with_ident(ty, "Option".into())
}

pub(crate) fn each_method(field: &syn::Field) -> std::option::Option<syn::Ident> {
    for attr in &field.attrs {
        if let std::result::Result::Ok(syn::Meta::List(syn::MetaList {
            ref path,
            ref nested,
            ..
        })) = attr.parse_meta()
        {
            if let Some(p) = path.segments.first() {
                if p.ident == "builder" {
                    if let Some(syn::NestedMeta::Meta(syn::Meta::NameValue(kv))) = nested.first() {
                        if kv.path.is_ident("each") {
                            if let syn::Lit::Str(ref ident_str) = kv.lit {
                                return std::option::Option::Some(syn::Ident::new(
                                    ident_str.value().as_str(),
                                    attr.span(),
                                ));
                            }
                        }
                    }
                }
            }
        }
    }
    std::option::Option::None
}

pub(crate) fn option_type_with_ident<'a>(
    ty: &'a syn::Type,
    ident: &str,
) -> std::option::Option<&'a syn::Type> {
    if let syn::Type::Path(syn::TypePath { ref path, .. }) = ty {
        if let std::option::Option::Some(seg) = path.segments.last() {
            if seg.ident == ident {
                if let syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments {
                    ref args,
                    ..
                }) = seg.arguments
                {
                    if let Some(syn::GenericArgument::Type(inner_type)) = args.first() {
                        return std::option::Option::Some(inner_type);
                    }
                }
            }
        }
    }
    std::option::Option::None
}

pub(crate) fn each_method_result(
    field: &syn::Field,
) -> syn::Result<std::option::Option<syn::Ident>> {
    for attr in &field.attrs {
        if let std::result::Result::Ok(syn::Meta::List(syn::MetaList {
            ref path,
            ref nested,
            ..
        })) = attr.parse_meta()
        {
            if let Some(p) = path.segments.first() {
                if p.ident == "builder" {
                    if let Some(syn::NestedMeta::Meta(syn::Meta::NameValue(kv))) = nested.first() {
                        if kv.path.is_ident("each") {
                            if let syn::Lit::Str(ref ident_str) = kv.lit {
                                return syn::Result::Ok(std::option::Option::Some(
                                    syn::Ident::new(ident_str.value().as_str(), attr.span()),
                                ));
                            }
                        } else {
                            if let std::result::Result::Ok(syn::Meta::List(ref list)) =
                                attr.parse_meta()
                            {
                                return syn::Result::Err(syn::Error::new_spanned(
                                    list,
                                    r#"expected `builder(each = "...")`"#,
                                ));
                            }
                        }
                    }
                }
            }
        }
    }
    syn::Result::Ok(std::option::Option::None)
}
