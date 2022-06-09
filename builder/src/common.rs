pub(crate) type FieldsType = syn::punctuated::Punctuated<syn::Field, syn::Token!(,)>;


pub(super) fn parse_fields(ast: &syn::DeriveInput) -> syn::Result<&FieldsType> {
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
    let err = syn::Error::new_spanned(ast, "parse fields error");
    syn::Result::Err(err)
} 