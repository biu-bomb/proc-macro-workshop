pub(super) fn solution(
    fields: &crate::common::FieldsType,
    builder_ident: &syn::Ident,
    origin_ident: &syn::Ident,
) -> proc_macro2::TokenStream {
    let mut token_stream = proc_macro2::TokenStream::new();

    // solution2
    let solution2_stream = solution2(fields, builder_ident, origin_ident);
    token_stream.extend(solution2_stream);

    // solution3
    let solution3_stream = solution3(fields, builder_ident, origin_ident);
    token_stream.extend(solution3_stream);

    // solution45
    let solution45_stream = solution45(fields, builder_ident, origin_ident);
    token_stream.extend(solution45_stream);

    token_stream
}

fn solution2(
    fields: &crate::common::FieldsType,
    builder_ident: &syn::Ident,
    origin_ident: &syn::Ident,
) -> proc_macro2::TokenStream {
    let struct_field_stream_vec: Vec<_> = fields
        .iter()
        .map(|f| {
            let ident = &f.ident;
            let ty = &f.ty;
            if let std::option::Option::Some(_) =
                crate::common::option_type_with_ident(ty, "Option")
            {
                quote::quote! {
                    pub #ident: #ty
                }
            } else if let std::option::Option::Some(_) =
                crate::common::option_type_with_ident(ty, "Vec")
            {
                quote::quote! {
                    pub #ident: #ty
                }
            } else {
                quote::quote! {
                    pub #ident: std::option::Option<#ty>
                }
            }
        })
        .collect();

    let construct_field_stream_vec: Vec<_> = fields
        .iter()
        .map(|f| {
            let ident = &f.ident;
            let ty = &f.ty;
            if let std::option::Option::Some(_) = crate::common::option_type_with_ident(ty, "Vec") {
                quote::quote! {
                    #ident: vec![]
                }
            } else {
                quote::quote! {
                    #ident: std::option::Option::None
                }
            }
        })
        .collect();
    quote::quote! {
        pub struct #builder_ident {
            #(
                #struct_field_stream_vec
            ),*
        }

        impl #origin_ident {
            pub fn builder() -> #builder_ident {
                #builder_ident {
                    #(
                        #construct_field_stream_vec
                    ),*
                }
            }
        }
    }
}

fn solution3(
    fields: &crate::common::FieldsType,
    builder_ident: &syn::Ident,
    _: &syn::Ident,
) -> proc_macro2::TokenStream {
    let setter_stream_vec: Vec<_> = fields
        .iter()
        .map(|f| {
            let ident = &f.ident;
            let ty = &f.ty;
            if let std::option::Option::Some(inner_type) =
                crate::common::option_type_with_ident(ty, "Option")
            {
                quote::quote! {
                    pub fn #ident(&mut self, #ident: #inner_type) -> &mut Self {
                        self.#ident = std::option::Option::Some(#ident);
                        self
                    }
                }
            } else if let std::option::Option::Some(inner_type) =
                crate::common::option_type_with_ident(ty, "Vec")
            {
                if let std::option::Option::Some(ref each_method_ident) =
                    crate::common::each_method(f)
                {
                    let mut each_method_stream = quote::quote! {
                        pub fn #each_method_ident(&mut self, #ident: #inner_type) -> &mut Self {
                            self.#ident.push(#ident);
                            self
                        }
                    };
                    if ident.as_ref().unwrap() != each_method_ident {
                        let origin_setter_stream = quote::quote! {
                            pub fn #ident(&mut self, #ident: #ty) -> &mut Self {
                                self.#ident = #ident;
                                self
                            }
                        };
                        each_method_stream.extend(origin_setter_stream);
                    }
                    each_method_stream
                } else {
                    quote::quote! {
                        pub fn #ident(&mut self, #ident: #ty) -> &mut Self {
                            self.#ident = #ident;
                            self
                        }
                    }
                }
            } else {
                quote::quote! {
                    pub fn #ident(&mut self, #ident: #ty) -> &mut Self {
                        self.#ident = std::option::Option::Some(#ident);
                        self
                    }
                }
            }
        })
        .collect();
    quote::quote! {
        impl #builder_ident {
            #(
                #setter_stream_vec
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
        .filter(|f| {
            let option_field = crate::common::option_type_with_ident(&f.ty, "Option");
            let vec_field = crate::common::option_type_with_ident(&f.ty, "Vec");
            option_field.is_none() && vec_field.is_none()
        })
        .map(|f| {
            let ident = &f.ident;
            quote::quote! {
                if self.#ident.is_none() {
                    let err = format!("field {} is missing", stringify!(#ident));
                    return std::result::Result::Err(err.into());
                }
            }
        })
        .collect();

    let construct_stream: Vec<_> = fields
        .iter()
        .map(|f| {
            let ident = &f.ident;
            let ty = &f.ty;
            if let std::option::Option::Some(_) =
                crate::common::option_type_with_ident(ty, "Option")
            {
                quote::quote! {
                    #ident: self.#ident.clone()
                }
            } else if let std::option::Option::Some(_) =
                crate::common::option_type_with_ident(ty, "Vec")
            {
                quote::quote! {
                    #ident: self.#ident.clone()
                }
            } else {
                quote::quote! {
                    #ident: self.#ident.clone().unwrap()
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
                        #construct_stream
                    ),*
                };
                std::result::Result::Ok(res)
            }
        }
    }
}
