pub(super) fn solution(
    fields: &crate::common::FieldsType,
    builder_ident: &syn::Ident,
    origin_ident: &syn::Ident,
) -> proc_macro2::TokenStream {
    let mut token_stream = proc_macro2::TokenStream::new();

    // solution2
    let refactor_solution2_stream = solution2(fields, builder_ident, origin_ident);
    token_stream.extend(refactor_solution2_stream);
    // solution3
    let refactor_solution3_stream = solution3(fields, builder_ident, origin_ident);
    token_stream.extend(refactor_solution3_stream);
    // solution45
    let refactor_solution45_stream = solution45(fields, builder_ident, origin_ident);
    token_stream.extend(refactor_solution45_stream);
    return token_stream;
}

fn solution2(
    fields: &crate::common::FieldsType,
    builder_ident: &syn::Ident,
    origin_ident: &syn::Ident,
) -> proc_macro2::TokenStream {
    let builder_fields_stream_vec: Vec<_> = fields
        .iter()
        .map(|f| {
            let ident = &f.ident;
            let ty = &f.ty;
            match crate::common::option_type(ty) {
                std::option::Option::Some(inner_type) => {
                    quote::quote! {
                        pub #ident: std::option::Option<#inner_type>
                    }
                }
                std::option::Option::None => {
                    quote::quote! {
                        pub #ident: std::option::Option<#ty>
                    }
                }
            }
        })
        .collect();
    let idents: Vec<_> = fields.iter().map(|f| &f.ident).collect();
    quote::quote! {
        pub struct #builder_ident {
            #(
                #builder_fields_stream_vec
            ),*
        }

        impl #origin_ident {
            pub fn builder() -> #builder_ident {
                #builder_ident {
                    #(
                        #idents: std::option::Option::None
                    ),*
                }
            }
        }
    }
}

fn solution3(
    fields: &crate::common::FieldsType,
    builder_ident: &syn::Ident,
    _origin_ident: &syn::Ident,
) -> proc_macro2::TokenStream {
    let setter_method_stream_vec: Vec<_> = fields
        .iter()
        .map(|f| {
            let ident = &f.ident;
            let ty = &f.ty;
            match crate::common::option_type(ty) {
                std::option::Option::Some(inner_type) => {
                    quote::quote! {
                        pub fn #ident(&mut self, #ident: #inner_type) -> &mut Self {
                            self.#ident = std::option::Option::Some(#ident);
                            self
                        }
                    }
                }
                std::option::Option::None => {
                    quote::quote! {
                        pub fn #ident(&mut self, #ident: #ty) -> &mut Self {
                            self.#ident = std::option::Option::Some(#ident);
                            self
                        }
                    }
                }
            }
        })
        .collect();

    quote::quote! {
        impl #builder_ident {
            #(
                #setter_method_stream_vec
            )*
        }
    }
}

fn solution45(
    fields: &crate::common::FieldsType,
    builder_ident: &syn::Ident,
    origin_ident: &syn::Ident,
) -> proc_macro2::TokenStream {
    let construct_if_stream_vec: Vec<_> = fields
        .iter()
        .map(|f| {
            let ident = &f.ident;
            let ty = &f.ty;
            if let std::option::Option::None = crate::common::option_type(ty) {
                return std::option::Option::Some(quote::quote! {
                    if self.#ident.is_none() {
                        let err = std::format!("field {} missing", std::stringify!(#ident));
                        return std::result::Result::Err(err.into());
                    }
                });
            }
            std::option::Option::None
        })
        .filter(|e| e.is_some())
        .collect();
    let construct_instance_stream_vec: Vec<_> = fields
        .iter()
        .map(|f| {
            let ident = &f.ident;
            let ty = &f.ty;
            match crate::common::option_type(ty) {
                std::option::Option::Some(_) => {
                    quote::quote! {
                        #ident: self.#ident.clone()
                    }
                }
                std::option::Option::None => {
                    quote::quote! {
                        #ident: self.#ident.clone().unwrap()
                    }
                }
            }
        })
        .collect();
    quote::quote! {
        impl #builder_ident {
            pub fn build(&self) -> std::result::Result<#origin_ident, std::boxed::Box<dyn std::error::Error>> {
                #(
                    #construct_if_stream_vec
                )*
                let res = #origin_ident {
                    #(
                       #construct_instance_stream_vec
                    ),*
                };
                std::result::Result::Ok(res)
            }
        }
    }
}
