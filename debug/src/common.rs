use syn::visit::Visit;

pub(crate) type FieldsType = syn::punctuated::Punctuated<syn::Field, syn::Token!(,)>;

pub(crate) fn parse_fields(ast: &syn::DeriveInput) -> syn::Result<&FieldsType> {
    if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(syn::FieldsNamed { ref named, .. }),
        ..
    }) = ast.data
    {
        return syn::Result::Ok(named);
    }
    syn::Result::Err(syn::Error::new_spanned(ast, "parse fields error"))
}

pub(crate) fn parse_format(
    field: &syn::Field,
) -> syn::Result<std::option::Option<std::string::String>> {
    for attr in field.attrs.iter() {
        if let std::result::Result::Ok(syn::Meta::NameValue(syn::MetaNameValue {
            ref path,
            ref lit,
            ..
        })) = attr.parse_meta()
        {
            if path.is_ident("debug") {
                if let syn::Lit::Str(ref ident_str) = lit {
                    return syn::Result::Ok(std::option::Option::Some(
                        ident_str.value().to_string(),
                    ));
                }
            }
        }
    }
    syn::Result::Ok(std::option::Option::None)
}

pub(crate) fn parse_generic_type(ast: &syn::DeriveInput) -> syn::Generics {
    return ast.generics.clone();
}

pub(crate) fn parse_phantom_generic_type_name(
    field: &syn::Field,
) -> syn::Result<std::option::Option<std::string::String>> {
    if let syn::Type::Path(syn::TypePath {
        path: syn::Path { ref segments, .. },
        ..
    }) = field.ty
    {
        if let std::option::Option::Some(syn::PathSegment {
            ref ident,
            ref arguments,
            ..
        }) = segments.last()
        {
            if ident == "PhantomData" {
                if let syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments {
                    ref args,
                    ..
                }) = arguments
                {
                    if let std::option::Option::Some(syn::GenericArgument::Type(syn::Type::Path(
                        p,
                    ))) = args.first()
                    {
                        if let std::option::Option::Some(gp) = p.path.segments.first() {
                            return syn::Result::Ok(std::option::Option::Some(gp.ident.to_string()));
                        }
                    }
                }
            }
        }
    }

    syn::Result::Ok(std::option::Option::None)
}

pub(crate) fn parse_field_type_name(
    field: &syn::Field,
) -> syn::Result<std::option::Option<std::string::String>> {
    if let syn::Type::Path(syn::TypePath {
        path: syn::Path { ref segments, .. },
        ..
    }) = field.ty
    {
        if let std::option::Option::Some(syn::PathSegment { ref ident, .. }) = segments.last() {
            return syn::Result::Ok(std::option::Option::Some(ident.to_string()));
        }
    }
    syn::Result::Ok(std::option::Option::None)
}

struct TypePathVisitor {
    interst_generic_type_names: Vec<String>,
    associated_type_names: std::collections::HashMap<String, Vec<syn::TypePath>>,
}

impl<'ast> syn::visit::Visit<'ast> for TypePathVisitor {
    fn visit_type_path(&mut self, node: &'ast syn::TypePath) {
        if node.path.segments.len() > 1 {
            let generic_type_name = node.path.segments[0].ident.to_string();
            if self.interst_generic_type_names.contains(&generic_type_name) {
                self.associated_type_names
                    .entry(generic_type_name)
                    .or_insert(vec![])
                    .push(node.clone());
            }
        }
        syn::visit::visit_type_path(self, node);
    }
}

pub(crate) fn parse_generic_associated_types(
    ast: &syn::DeriveInput,
) -> std::collections::HashMap<String, Vec<syn::TypePath>> {
    let origin_generic_type_names: Vec<String> = ast
        .generics
        .params
        .iter()
        .filter_map(|f| {
            if let syn::GenericParam::Type(t) = f {
                return Some(t.ident.to_string());
            }
            return None;
        })
        .collect();
    let mut visitor = TypePathVisitor {
        interst_generic_type_names: origin_generic_type_names,
        associated_type_names: std::collections::HashMap::new(),
    };
    visitor.visit_derive_input(ast);
    return visitor.associated_type_names;
}

pub(crate) fn parse_customer_debug(
    ast: &syn::DeriveInput,
) -> syn::Result<std::option::Option<std::string::String>> {
    for attr in ast.attrs.iter() {
        if let syn::Result::Ok(syn::Meta::List(syn::MetaList {
            ref path,
            ref nested,
            ..
        })) = attr.parse_meta()
        {
            if path.is_ident("debug") {
                if let std::option::Option::Some(syn::NestedMeta::Meta(syn::Meta::NameValue(
                    syn::MetaNameValue {
                        ref path, ref lit, ..
                    },
                ))) = nested.first()
                {
                    if path.is_ident("bound") {
                        if let syn::Lit::Str(ref customer_where_clause) = lit {
                            return syn::Result::Ok(std::option::Option::Some(
                                customer_where_clause.value().to_string(),
                            ));
                        }
                    }
                }
            }
        }
    }
    syn::Result::Ok(std::option::Option::None)
}
