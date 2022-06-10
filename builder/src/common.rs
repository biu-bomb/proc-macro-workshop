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

pub(crate) fn option_type(ty: &syn::Type) -> std::option::Option<&syn::Type> {
    if let syn::Type::Path(
        syn::TypePath {
            path: syn::Path{
                segments,
                ..
            },
            ..
        }
    ) = ty {
        if let Some(seg) = segments.last() {
            if seg.ident == "Option" {
                if let syn::PathArguments::AngleBracketed(
                    syn::AngleBracketedGenericArguments{
                        ref args,
                    ..
                }) = seg.arguments {
                    if let Some(syn::GenericArgument::Type(inner_type)) = args.first() {
                        return std::option::Option::Some(inner_type);
                    }
                }
            }
        }
    }
    std::option::Option::None
}