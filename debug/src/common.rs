pub(crate) type FielsType = syn::punctuated::Punctuated<syn::Field, syn::Token!(,)>;

pub(crate) fn parse_fields(ast: &syn::DeriveInput) -> syn::Result<&FielsType> {
    if let syn::Data::Struct(
        syn::DataStruct {
            fields: syn::Fields::Named(
                syn::FieldsNamed {
                    ref named,
                    ..
                }
            ),
            ..
        }
    ) = ast.data {
        return syn::Result::Ok(named);
    }
    syn::Result::Err(syn::Error::new_spanned(ast, "parse fields error"))
}

pub(crate) fn parse_format(field: &syn::Field) -> syn::Result<std::option::Option<std::string::String>> {
    for attr in field.attrs.iter() {
        if let std::result::Result::Ok(
            syn::Meta::NameValue(
                syn::MetaNameValue{
                    ref path,
                    ref lit,
                    ..
                }
            )
        ) = attr.parse_meta() {
            if path.is_ident("debug") {
                if let syn::Lit::Str(ref ident_str) = lit {
                    return syn::Result::Ok(std::option::Option::Some(ident_str.value().to_string()));
                }
            }
        }
    }
    syn::Result::Ok(std::option::Option::None)
}

pub(crate) fn parse_generic_type(ast: &syn::DeriveInput) -> syn::Generics {
    return ast.generics.clone();
}
