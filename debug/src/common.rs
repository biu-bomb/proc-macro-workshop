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
