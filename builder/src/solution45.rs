pub(super) fn solution(fields: &crate::common::FieldsType, builder_ident: &syn::Ident, origin_ident: &syn::Ident) -> proc_macro2::TokenStream {
    let idents: Vec<_> = fields.iter().map(|f| &f.ident).collect();
    quote::quote! {
        impl #builder_ident {
            pub fn build(&self) -> std::result::Result<#origin_ident, std::boxed::Box<dyn std::error::Error>> {
                #(
                    if self.#idents.is_none() {
                        let err = std::format!("field {} missing", std::stringify!(#idents));
                        return std::result::Result::Err(err.into());
                    }
                )*
                let res = #origin_ident {
                    #(
                        #idents: self.#idents.clone().unwrap()
                    ),*
                };
                std::result::Result::Ok(res)
            }
        }
    }
}