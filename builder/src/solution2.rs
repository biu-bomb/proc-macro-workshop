pub(super) fn solution(
    fields: &crate::common::FieldsType,
    origin_ident: &syn::Ident,
    builder_ident: &syn::Ident,
) -> proc_macro2::TokenStream {
    let idents: Vec<_> = fields.iter().map(|f| &f.ident).collect();
    let tys: Vec<_> = fields.iter().map(|f| &f.ty).collect();
    quote::quote! {
        pub struct #builder_ident {
            #(
                pub #idents: std::option::Option<#tys>
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
